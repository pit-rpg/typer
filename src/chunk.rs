use std::ops::Add;
use super::units::*;
use super::rusttype_renderer::{is_line_break};
extern crate rusttype;


use self::rusttype::{ScaledGlyph, Font};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TextAlignHorizontal {
	Left,
	Right,
	Center,
	Justify,
}

#[derive(Debug)]
pub enum FormatChunks {
	Chunk(FormatChunk),
	String(String),
}


#[derive(Debug)]
pub struct FormatChunk {
	pub font_size: usize,
	pub line_height: f32,
	pub color: ColorRGBA,
	// pub text_align: TextAlignHorizontal,
	pub font: Option<String>,
	// pub width: Option<usize>,
	pub chunks: Vec<FormatChunks>,
}

#[derive(Debug)]
pub struct FormatBlock {
	pub text_align: TextAlignHorizontal,
	pub width: f32,
	pub height: f32,
	pub x: f32,
	pub y: f32,
	pub chunk: FormatChunk,
}


#[derive(Debug)]
pub struct Layout<'a> {
	pub blocks: Vec<(FormatBlock, RenderBlock<'a>)>,
	// pub render_blocks: Vec<RenderBlock<'a>>,
}


impl FormatChunk {

	pub fn new() -> Self {
		Self {
			font_size: 10,
			line_height: 1.0,
			color: [0, 0, 0, 255],
			font: None,
			chunks: Vec::new(),
		}
	}

	pub fn set_attribute(&mut self, key: &str, val: &str) {
		match key {
			"font-size" 	=> { self.font_size = val.parse::<usize>().unwrap() }
			"line-height" 	=> { self.line_height = val.parse::<f32>().unwrap() }
			"font" 			=> { self.font = Some(val.to_string()) }
			"color" 			=> {
				let err = &format!("wrong color: {}", val);
				if val.starts_with('#') && val.len() == 7 {
					self.color = [
						u8::from_str_radix(val.get(1..3).expect(err), 16).expect(err),
						u8::from_str_radix(val.get(3..5).expect(err), 16).expect(err),
						u8::from_str_radix(val.get(5..).expect(err), 16).expect(err),
						255
					];
				} else if val.starts_with('#') && val.len() == 9 {
					self.color = [
						u8::from_str_radix(val.get(1..3).expect(err), 16).expect(err),
						u8::from_str_radix(val.get(3..5).expect(err), 16).expect(err),
						u8::from_str_radix(val.get(5..7).expect(err), 16).expect(err),
						u8::from_str_radix(val.get(7..).expect(err), 16).expect(err)
					];
				}
			}
			_ => {
				println!("unknown attribute: {}", key);
			}
		}
	}


	pub fn new_empty(&self) -> Self {
		let mut res = Self {
			font_size: self.font_size,
			line_height: self.line_height,
			color: self.color,
			// text_align: self.text_align,
			font: None,
			// width: None,
			chunks: Vec::new(),
		};

		if let Some(font) = &self.font {
			res.font = Some(font.clone());
		}

		res
	}
}





impl FormatBlock {

	pub fn new() -> Self {
		Self {
			text_align: TextAlignHorizontal::Left,
			width: 0.0,
			height: 0.0,
			x: 0.0,
			y: 0.0,
			chunk: FormatChunk::new(),
		}
	}

	pub fn set_attribute(&mut self, key: &str, val: &str) {
		match key {
			"text-align" 			=> {
				match val {
					"left"|"LEFT" 		=> { self.text_align = TextAlignHorizontal::Left }
					"right"|"RIGHT" 	=> { self.text_align = TextAlignHorizontal::Right }
					"center"|"CENTER" 	=> { self.text_align = TextAlignHorizontal::Center }
					"justify"|"JUSTIFY" => { self.text_align = TextAlignHorizontal::Justify }
					_ => {}
				}
			}

			"width" 		=> { self.width = val.parse::<f32>().unwrap().abs() }
			"height" 		=> { self.width = val.parse::<f32>().unwrap().abs() }
			"x" 			=> { self.width = val.parse::<f32>().unwrap() }
			"y" 			=> { self.width = val.parse::<f32>().unwrap() }

			_ => {
				println!("unknown attribute: {}", key);
			}
		}
	}


	pub fn new_empty(&self) -> Self {
		let mut res = Self {
			width: self.width,
			height: self.height,
			x: self.x,
			y: self.y,
			text_align: self.text_align,
			chunk: self.chunk.new_empty(),
		};

		res
	}
}



#[derive(Debug)]
pub struct Line<'a> {
	pub width: f32,
	pub descent: f32,
	pub height: f32,
	pub chars_width: f32,
	pub glyphs: Vec<(ScaledGlyph<'a>, &'a FormatChunk, char)>,
	// text_align: TextAlignHorizontal,
	// x: f32,
	// y: f32,
}

impl <'a> Line<'a> {
	pub fn new() -> Self {
		Self {
			width: 0.0,
			descent: 0.0,
			height: 0.0,
			chars_width: 0.0,
			glyphs: Vec::new(),
		}
	}

}



#[derive(Debug)]
pub struct RenderBlock<'a> {
	// pub format_block: &'a FormatBlock,
	pub lines: Vec<Line<'a>>,
	// pub text_align: TextAlignHorizontal,
	// pub width: f32,
	// pub height: f32,
	// pub x: f32,
	// pub y: f32,

	// pub chunk: FormatChunk,
}

impl <'a> RenderBlock<'a> {

	pub fn new () -> Self {
		let mut b = RenderBlock{
			lines: Vec::new()
		};
		b.add_line();
		b
	}

	pub fn add_line(&mut self) {
		self.lines.push(Line::new());
	}

	pub fn get_line(&mut self) -> &mut Line<'a> {
		self.lines.last_mut().unwrap()
	}

	// pub fn add_symbol(&mut self, shunk: &FormatChunk, c: char, font: &Font ) {

	// 	if is_line_break(c) {
	// 		self.add_line();
	// 		return;
	// 	}

	// 	let line = self.lines.last_mut().unwrap();

	// 	// if self.width == 0.0 {

	// 	// }


	// 	print!("{}", c);
	// }
}
