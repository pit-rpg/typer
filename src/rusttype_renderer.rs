extern crate rusttype;

use std::path::PathBuf;
use std::char;
use std::cmp::Ordering;
use std::fs::{File};
use std::io::Read;
use self::rusttype::{Scale, point, Rect, Font};
use chunk::{FormatBlock, Layout, TextAlignHorizontal};
use img_buffer::{ImgBufferTrait};


pub struct TextRenderer {}

impl TextRenderer {
	pub fn new () -> Self {
		Self {}
	}


	pub fn load_fonts <'a>(data: Vec<(String, PathBuf)>) -> Vec<(String, Font<'a>)> {
		let mut result = Vec::with_capacity(data.len());
		for (name, path) in data {
			let mut f = File::open(path).expect("wrong font path");
			let mut buffer = Vec::new();
			f.read_to_end(&mut buffer).expect("cant read from font file");
			let font = Font::from_bytes(buffer).expect("Error constructing Font");
			result.push((name, font));
		}
		result
	}


	fn find_font<'a>(name: &Option<String>, fonts: &'a[(String, Font<'a>)] ) -> (Option<std::string::String>, &'a rusttype::Font<'a>) {
		match name {
			None => {(Some(fonts[0].0.clone()), &fonts[0].1)}
			Some(font_name) => {
				if let Some(font) = fonts
					.iter()
					.find(|(e_name, _)| e_name == font_name )
					{
						return (Some(font.0.clone()), &font.1);
					}
				(Some(fonts[0].0.clone()), &fonts[0].1)
			}
		}
	}


	pub fn format<'a>(format_blocks: Vec<FormatBlock>, dpi_factor: f32, fonts: &'a[(String, Font<'a>)]) -> Layout<'a> {

		let mut layout = Layout {
			blocks: Vec::with_capacity(format_blocks.len()),
			width:0.0,
			height:0.0,
			x:0.0,
			y:0.0,
		};

		let (mut current_font_name, mut font) = Self::find_font(&None, fonts);
		let mut scale = Scale::uniform(0.0);

		for block in format_blocks {

			let mut last_wight_space = None;
			let mut line_width = 0.0;
			let mut prev_glyph_id = None;

			let mut render_block = block.to_render_block();

			for (chunk, str_data) in block.chunk.iter() {

				if is_font_need_update(&chunk.font, &current_font_name) {
					let (f,n) = Self::find_font(&chunk.font, fonts);
					current_font_name = f;
					font = n;
					prev_glyph_id = None;
				}

				scale = Scale::uniform(chunk.font_size as f32 * dpi_factor);
				let v_metrics = font.v_metrics(scale);

				for symbol in str_data.chars() {
					if is_line_break(symbol) {
						render_block.get_last_line().force_break = true;
						render_block.add_line();
						prev_glyph_id = None;
						line_width = 0.0;
						continue;
					}

					{
						let line = render_block.get_last_line();
						line.height = line.height.max( (v_metrics.line_gap + v_metrics.ascent) * chunk.line_height );
						line.descent = line.descent.min( v_metrics.descent );
					}

					let base_glyph = font.glyph(symbol);
					let mut glyph = base_glyph.scaled(scale);
					let h_metrics = glyph.h_metrics();
					let mut symbol_width = h_metrics.advance_width;

					if let Some(id) = prev_glyph_id {
						symbol_width += font.pair_kerning(scale, id, glyph.id());
					}
					prev_glyph_id = Some(glyph.id());

					if block.width == 0.0 {
						render_block.get_last_line().glyphs.push((glyph, chunk.get_render_chunk(), symbol, symbol_width));
						continue;
					} else if line_width+symbol_width > block.width {
						render_block.add_line();
						prev_glyph_id = None;
						line_width = 0.0;
						
						if is_can_line_break(symbol) {
							last_wight_space = None;
							continue;
						}

						if !block.break_word {
							if let Some(i) = last_wight_space {
								let mut vec = render_block.get_prev_line().glyphs
									.splice(i.., None)
									.skip(1)
									.collect();
								render_block.get_last_line().glyphs.append(&mut vec);
								line_width = render_block.get_last_line().glyphs
									.iter()
									.map(|(..,sw)| sw )
									.sum();
								last_wight_space = None;
							}
						}

						render_block.get_last_line().glyphs.push((glyph, chunk.get_render_chunk(), symbol, symbol_width));
						line_width += symbol_width;
					} else {
						if is_can_line_break(symbol) {
							if !block.break_word {
								last_wight_space = Some(render_block.get_last_line().glyphs.len());
							} else if line_width == 0.0 {continue;} 
						}

						render_block.get_last_line().glyphs.push((glyph, chunk.get_render_chunk(), symbol, symbol_width));
						line_width += symbol_width;
					}
				}

				{
					let mut prev_line_height = 0.0;
					if let Some(h) = render_block.lines
						.iter()
						.find(|l| l.height > 0.0) {
							prev_line_height = h.height;
						}
					render_block.lines
						.iter_mut()
						.for_each(|line| {
							if line.height == 0.0 {line.height = prev_line_height} else {prev_line_height = line.height}
						})
				}

				{
					let width = render_block.lines
						.iter_mut()
						.map(|line| -> f32 {
							let width = line.glyphs
								.iter()
								.map(|e| e.3)
								.sum();
							line.width = width;
							width
						})
						.max_by(|a, b| if a > b {Ordering::Greater} else {Ordering::Less})
						.unwrap();
					render_block.width = width;
					let mut height = render_block.lines
						.iter()
						.map(|line| line.height )
						.sum();
					height += - render_block.get_last_line().descent;
					render_block.height = height;
				}
			}

			layout.blocks.push((block, render_block));
		}

		layout
	}


	pub fn render<T: ImgBufferTrait>( layout: &Layout, buffer: &mut T )
	{
		let mut caret = point(0.0, 0.0);
		let buffer_width = buffer.width() as i32;
		let buffer_height = buffer.height() as i32;

		for ( f_block, r_block ) in layout.blocks.iter() {
			let offset = point(f_block.x - layout.x, f_block.y - layout.y);

			caret.y = offset.y;

			let lines_count = r_block.lines.len();

			for (i, line) in r_block.lines.iter().enumerate() {
				caret.y += line.height + line.descent;

				let mut space_inc = 0.0; 
				
				match f_block.text_align {
					TextAlignHorizontal::Right => {caret.x = offset.x + f_block.width - line.width;}
					TextAlignHorizontal::Center => {caret.x = offset.x + ((f_block.width - line.width)/2.0);}
					TextAlignHorizontal::Justify => {
						caret.x = offset.x;
						if !line.force_break && i != lines_count-1 {
							let c = line.glyphs
								.iter()
								.filter( |(_,_,symbol,_)| *symbol == ' ')
								.count();
							space_inc = (f_block.width - line.width) / (c as f32);
						}
					} 
					_ => {caret.x = offset.x;}
				}

				for (scaled_glyph, chunk, symbol, symbol_width) in line.glyphs.iter() {
					if *symbol == ' ' { caret.x += space_inc };

					let positioned_glyph = scaled_glyph.clone().positioned(caret);

					if let Some(bounding_box) = positioned_glyph.pixel_bounding_box() {
						if can_draw(bounding_box, buffer_width, buffer_height) {
							positioned_glyph.draw(|x, y, v| {
								let x = bounding_box.min.x + (x as i32);
								let y = bounding_box.min.y + (y as i32);

								if x < 0 {return};
								if y < 0 {return};

								buffer.blend_pixel(x as usize, y as usize, &chunk.color, v);
							});
						}
					}
					caret.x += symbol_width;
				}
				caret.y -= line.descent;
			}
		}
	}

}


fn can_draw(rect: Rect<i32>, w:i32, h:i32) -> bool {
	if
		rect.max.x < 0 ||
		rect.max.x > w ||
		rect.max.y < 0 ||
		rect.max.y > h
		{false} else {true}
}

fn is_font_need_update(a: &Option<String>, b: &Option<String>) -> bool {
	match (a, b) {
		(None, None) => false,
		(Some(_), None) => true,
		(None, Some(_)) => true,
		(Some(s_a), Some(s_b)) => s_a != s_b,
	}
}


// TODO: if need 
// http://www.fileformat.info/info/unicode/category/Zs/list.htm
// https://en.wikipedia.org/wiki/Whitespace_character
// http://www.unicode.org/Public/UNIDATA/UnicodeData.txt
// http://unicode.org/reports/tr14/
// or 
// https://crates.io/crates/xi-unicode

static LINE_BREAK: &[char] = &['↵', '', '', '\n', '', ' ', ' '];

const CAN_LINE_BREAK: &[char] = &[
	' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '　'
];


pub fn is_line_break(c: char) -> bool {
	LINE_BREAK
		.iter()
		.find(|e| **e == c)
		.is_some()
}

pub fn is_can_line_break(c: char) -> bool {
	CAN_LINE_BREAK
		.iter()
		.find(|e| **e == c)
		.is_some()
}
