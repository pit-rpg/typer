extern crate rusttype;
extern crate unicode_normalization;

// use self::rusttype::gpu_cache::Cache;
use self::rusttype::*;
use self::unicode_normalization::UnicodeNormalization;
// use std::usize::MAX;
use super::Chunk;
// use std::convert::TryFrom;
use std::char::from_u32_unchecked;
use std::char;


use units::ColorRGBA;

pub struct TextRenderer {
	pub background: ColorRGBA,
	pub width: usize,
	pub height: usize,
	pub break_word: bool,
	pub padding: (usize, usize, usize, usize),

	// max_height: usize,
}


impl TextRenderer {
	pub fn new () -> Self {
		Self {
			background: ColorRGBA{r:0, g:0, b:0, a:0},
			width: 0,
			height: 0,
			break_word: false,
			padding: (0, 0, 0, 0),
		}
	}


	fn find_font<'a>(name: &Option<String>, fonts: &'a [Font]) -> &'a Font<'a> {
		match name {
			None => {&fonts[0]}
			Some(font_name) => {
				for font in fonts {
					for (data, _, _) in font.font_name_strings() {
						let res: String = data
							.iter()
							.map(|e| *e as char)
							.collect();
						// println!("{}", res);
						if res == *font_name {return font;}
					}
				}
				&fonts[0]
			}
		}
	}




	pub fn render(&self, chunks: Vec<Chunk>, fonts: &[Font], dpi_factor: f32) {

		// calc lines
		let mut font = TextRenderer::find_font(&chunks[0].font, fonts);
		let mut current_font_name = &chunks[0].font;
		let mut v_metrics;
		let mut scale = Scale::uniform(0.0);
		let mut advance_height:f32 = 0.0;
		let mut line_width = 0.0;
		let mut word_width = 0.0;

		let mut lines: Vec<(Vec<(ScaledGlyph)>, f32, f32)> = Vec::new();
		let mut current_line = Vec::new();
		let mut current_word: Vec<ScaledGlyph> = Vec::new();

		for chunk in chunks.iter() {

			if !eq_font(current_font_name, &chunk.font) {
				font = TextRenderer::find_font(&chunk.font, fonts);
				current_font_name = &chunk.font;
			}

			if let Some(font_size) = chunk.font_size {
				scale = Scale::uniform(font_size as f32 * dpi_factor);
				v_metrics = font.v_metrics(scale);
				advance_height = advance_height.max(v_metrics.ascent - v_metrics.descent + v_metrics.line_gap);
			}

			for letter in chunk.string.nfc() {
				let base_glyph = font.glyph(letter);
				let mut glyph = base_glyph.scaled(scale);

				let is_break = LINE_BREAK
					.iter()
					.find(|e| **e == letter)
					.is_some();
				if is_break {
					// new line
					lines.push((current_line, line_width, advance_height));
					current_line = Vec::new();
					line_width = 0.0;
					advance_height = 0.0;
					// /new line
				}
				let h_metrics = glyph.h_metrics();
				line_width += h_metrics.advance_width;

				if self.width != 0 {
					if self.break_word {
						let is_break = CAN_LINE_BREAK
							.iter()
							.find(|e| **e == letter)
							.is_some();

						if line_width > self.width as f32 && is_break {
							current_line.append(&mut current_word);
							current_word = Vec::new();
							word_width = 0.0;

							// new line
							lines.push((current_line, line_width - h_metrics.advance_width, advance_height));
							current_line = Vec::new();
							line_width = 0.0;
							advance_height = 0.0;
							// /new line
						} else if line_width > self.width as f32 {
							// new line
							lines.push((current_line, line_width - h_metrics.advance_width - word_width, advance_height));
							current_line = Vec::new();
							line_width = 0.0;
							advance_height = 0.0;
							// /new line

							current_word.push(glyph);
							word_width+=h_metrics.advance_width;
						} else if is_break {
							current_line.append(&mut current_word);
							current_line.push(glyph);
							current_word = Vec::new();
							word_width = 0.0;
						} else {
							current_word.push(glyph);
							word_width += h_metrics.advance_width;
						}
					} else {
						if  line_width > self.width as f32 {
							// new line
							lines.push((current_line, line_width - h_metrics.advance_width, advance_height));
							current_line = Vec::new();
							line_width = 0.0;
							advance_height = 0.0;
							// /new line
						}
						current_line.push(glyph);
					}
				} else {
					current_line.push(glyph);
				}
			}
		}

		lines.push((current_line, line_width, advance_height));

		// set positions;
		// 1 get line sizes
		// let mut lines = lines
		// 	.iter()
		// 	.map(|line| {
		// 		let height = 0.0;
		// 		let width = 0.0;
		// 		for glyph in line.iter() {
		// 			width = width.max(glyph.get)
		// 		}
		// 	});

		for (_, w,h) in lines.iter() {
			println!("{}x{}", w.ceil() as usize, h.ceil() as usize);
		}

		let caret = point(0.0, 0.0);
		let mut img_width = self.width;
		let mut img_height = self.height;

		if img_width == 0 {
			let width: f32 = lines.iter().map(|(_,w,_)| -> &f32 {w} ).sum();
			img_width =  width.ceil() as usize;
		}
		if img_height == 0 {
			let width: f32 = lines.iter().map(|(_,_,h)| -> &f32 {h} ).sum();
			img_height =  width.ceil() as usize;
		}

	}

}



fn eq_font<'a>(a: &Option<String>, b: &Option<String>) -> bool {
	match (a, b) {
		(Some(na), Some(nb)) => {na==nb}
		(Some(na), None) => {false}
		(None, Some(na),) => {false}
		_ => false
	}
}

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
