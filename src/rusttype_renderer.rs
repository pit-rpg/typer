extern crate rusttype;

use std::path::PathBuf;
use std::char;
use std::cmp::Ordering;
use self::rusttype::{Scale, point};
use super::*;


pub struct TextRenderer {}


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


	fn find_font<'a>(name: &Option<String>, fonts: &'a[(String, Font<'a>)] ) -> &'a Font<'a> {
		match name {
			None => {&fonts[0].1}
			Some(font_name) => {
				let res = fonts
					.iter()
					.find(|(name, _)| name == font_name );
				if let Some(font) = res {
					return &font.1;
				}
				&fonts[0].1
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

		let mut current_font_name = Some("".to_string());
		let mut font = Self::find_font(&current_font_name, fonts);
		let mut scale = Scale::uniform(0.0);

        for block in format_blocks {

			let mut last_wight_space = None;
			let mut line_width = 0.0;
			let mut prev_glyph_id = None;

			let mut render_block = block.to_render_block();

			for (chunk, str_data) in block.chunk.iter() {
				if chunk.font != current_font_name {
					font = Self::find_font(&current_font_name, fonts);
					current_font_name = chunk.font.clone();
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
						line.descent = line.descent.min(v_metrics.descent);
					}

					let base_glyph = font.glyph(symbol);
					let mut glyph = base_glyph.scaled(scale);
					let h_metrics = glyph.h_metrics();
					let mut symbol_width = h_metrics.advance_width;
					// println!("{}", h_metrics.left_side_bearing);

					if let Some(id) = prev_glyph_id {
						// symbol_width += font.pair_kerning(scale, id, glyph.id());
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


	pub fn render( layout: &Layout, buffer: &mut ImgBuffer ) {
		println!("================== LAYOUT ==================");

		let mut caret = point(0.0, 0.0);
		let buffer_width = buffer.width as i32;
		let buffer_height = buffer.height as i32;

		for ( _f_block, r_block ) in layout.blocks.iter() {
			let offset = point(r_block.x - layout.x, r_block.y - layout.y);

			caret.y = offset.y;

			let lines_count = r_block.lines.len();

			for (i, line) in r_block.lines.iter().enumerate() {
				caret.y += line.height + line.descent;
				let mut space_inc = 0.0; 
				
				match r_block.text_align {
					TextAlignHorizontal::Right => {caret.x = offset.x + r_block.width - line.width;}
					TextAlignHorizontal::Center => {caret.x = (offset.x + r_block.width - line.width)/2.0;}
					TextAlignHorizontal::Justify => {
						caret.x = offset.x;
						if !line.force_break && i != lines_count-1 {
							let c = line.glyphs
								.iter()
								.filter( |(_,_,symbol,_)| *symbol == ' ')
								.count();
							space_inc = (r_block.width - line.width) / (c as f32);
						}
					} 
					_ => {caret.x = offset.x;}
				}
				
				for (scaled_glyph, chunk, symbol, symbol_width) in line.glyphs.iter() {
					print!("{}", symbol);
					if *symbol == ' ' { caret.x += space_inc };

					let positioned_glyph = scaled_glyph.clone().positioned(caret);

					if let Some(bounding_box) = positioned_glyph.pixel_bounding_box() {
						positioned_glyph.draw(|x, y, v| {
							let x = bounding_box.min.x + (x as i32);
							let y = bounding_box.min.y + (y as i32);

							if x < 0 {return};
							if y < 0 {return};
							if x >= buffer_width {return};
							if y >= buffer_height {return};

							buffer.blend_pixel(x as usize, y as usize, &chunk.color, v);
						});
					}
					caret.x += symbol_width;
				}

				caret.y -= line.descent;
				println!("=======");
			}
		}
	}

}



// fn eq_font<'a>(a: &Option<String>, b: &Option<String>) -> bool {
// 	match (a, b) {
// 		(Some(na), Some(nb)) => {na == nb}
// 		(Some(_), None) => {false}
// 		(None, Some(_),) => {false}
// 		_ => false
// 	}
// }

// http://www.fileformat.info/info/unicode/category/Zs/list.htm
// Unicode Characters in the 'Separator, Space' Category
// https://en.wikipedia.org/wiki/Whitespace_character
// http://www.unicode.org/Public/UNIDATA/UnicodeData.txt


static LINE_BREAK: &[char] = &['↵', '', '', '\n', '', ' ', ' '];

const CAN_LINE_BREAK: &[char] = &[
	' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '　'
	// 0x0020, as char ,	// SPACE;Zs;0;WS;;;;;N;;;;;
	// 0x1680, as char ,	// OGHAM SPACE MARK;Zs;0;WS;;;;;N;;;;;
	// 0x2000, as char ,	// EN QUAD;Zs;0;WS;2002;;;;N;;;;;
	// 0x2001, as char ,	// EM QUAD;Zs;0;WS;2003;;;;N;;;;;
	// 0x2002, as char ,	// EN SPACE;Zs;0;WS;<compat> 0020;;;;N;;;;;
	// 0x2003, as char ,	// EM SPACE;Zs;0;WS;<compat> 0020;;;;N;;;;;
	// 0x2004, as char ,	// THREE-PER-EM SPACE;Zs;0;WS;<compat> 0020;;;;N;;;;;
	// 0x2005, as char ,	// FOUR-PER-EM SPACE;Zs;0;WS;<compat> 0020;;;;N;;;;;
	// 0x2006, as char ,	// SIX-PER-EM SPACE;Zs;0;WS;<compat> 0020;;;;N;;;;;
	// 0x2008, as char ,	// PUNCTUATION SPACE;Zs;0;WS;<compat> 0020;;;;N;;;;;
	// 0x2009, as char ,	// THIN SPACE;Zs;0;WS;<compat> 0020;;;;N;;;;;
	// 0x200A, as char ,	// HAIR SPACE;Zs;0;WS;<compat> 0020;;;;N;;;;;
	// 0x205F, as char ,	// MEDIUM MATHEMATICAL SPACE;Zs;0;WS;<compat> 0020;;;;N;;;;;
	// 0x3000, as char ,	// IDEOGRAPHIC SPACE;Zs;0;WS;<wide> 0020;;;;N;;;;;
];
