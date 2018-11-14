use super::units::*;
extern crate rusttype;

use self::rusttype::{ScaledGlyph};


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
pub struct RenderChunk {
	pub line_height: f32,
	pub color: ColorRGBA,
}


#[derive(Debug)]
pub struct Layout<'a> {
	pub blocks: Vec<(FormatBlock, RenderBlock<'a>)>,
	pub width: f32,
	pub height: f32,
	pub x: f32,
	pub y: f32,
}

impl <'a> Layout<'a> {

	pub fn calk_view(&mut self) {
		let mut width = - std::f32::MAX;
		let mut height = - std::f32::MAX;
		let mut x = - std::f32::MAX;
		let mut y = - std::f32::MAX;

		self.blocks
			.iter()
			.for_each(|(_, block)| {
				x = x.max(block.x);
				y = y.max(block.y);
				width = width.max(block.x + block.width);
				height = height.max(block.y + block.height);
			});

		self.width = width;
		self.height = height;
		self.x = x;
		self.y = y;
	}


	pub fn create_buffer(&mut self) -> Option<ImgBuffer> {
		if self.width < 1.0 && self.height < 1.00 {return None};
		Some(ImgBuffer::new(self.width.ceil() as usize, self.height.ceil() as usize, &[255,255,255,255]))
	}
}


#[derive(Debug)]
pub struct FormatChunk {
	pub font_size: usize,
	pub line_height: f32,
	pub color: ColorRGBA,
	pub font: Option<String>,
	pub chunks: Vec<FormatChunks>,
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
			font: None,
			chunks: Vec::new(),
		};

		if let Some(font) = &self.font {
			res.font = Some(font.clone());
		}

		res
	}


	pub fn iter(&self) -> FormatChunkIter {
		FormatChunkIter{
			index: 0,
			chunk: self,
			sub_iter: None,
		}
	}

	pub fn get_render_chunk (&self) -> RenderChunk {
		RenderChunk{
			line_height: self.line_height,
			color: self.color,
		}
	}
}


#[derive(Debug)]
pub struct FormatBlock {
	pub text_align: TextAlignHorizontal,
	pub break_word: bool,
	pub width: f32,
	pub height: f32,
	pub x: f32,
	pub y: f32,
	pub chunk: FormatChunk,
}


impl FormatBlock {

	pub fn new() -> Self {
		Self {
			text_align: TextAlignHorizontal::Left,
			break_word: false,
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
			"height" 		=> { self.height = val.parse::<f32>().unwrap().abs() }
			"x" 			=> { self.x = val.parse::<f32>().unwrap() }
			"y" 			=> { self.y = val.parse::<f32>().unwrap() }
			"break_word" 		=> {
				match val {
					"true"|"TRUE"|"1"|"yes" 	=> { self.break_word = true }
					"false"|"FALSE"|"0"|"no" 	=> { self.break_word = false }
					_ => {}
				}
			}
			_ => {
				println!("unknown attribute: {}", key);
			}
		}
	}


	pub fn new_empty(&self) -> Self {
		let res = Self {
			break_word: self.break_word,
			width: self.width,
			height: self.height,
			x: self.x,
			y: self.y,
			text_align: self.text_align,
			chunk: self.chunk.new_empty(),
		};

		res
	}

	pub fn to_render_block <'a> (&self) -> RenderBlock<'a> {
		let mut b = RenderBlock {
			text_align: self.text_align,
			width: self.width,
			height: self.height,
			x: self.x,
			y: self.y,
			lines: Vec::new(),
		};
		b.add_line();
		b
	}
}



#[derive(Debug)]
pub struct Line<'a> {
	pub width: f32,
	pub descent: f32,
	pub height: f32,
	pub chars_width: f32,
	pub force_break: bool,
	pub glyphs: Vec<(ScaledGlyph<'a>, RenderChunk, char, f32)>,
}

impl <'a> Line<'a> {
	pub fn new() -> Self {
		Self {
			width: 0.0,
			descent: 0.0,
			height: 0.0,
			chars_width: 0.0,
			force_break: false,
			glyphs: Vec::new(),
		}
	}

}


#[derive(Debug)]
pub struct RenderBlock<'a> {
	pub lines: Vec<Line<'a>>,
	pub text_align: TextAlignHorizontal,
	pub width: f32,
	pub height: f32,
	pub x: f32,
	pub y: f32,
}


impl <'a> RenderBlock<'a> {

	// pub fn new () -> Self {
	// 	let mut b = RenderBlock{
	// 		text_align: TextAlignHorizontal::Left,
	// 		lines: Vec::new(),
	// 		width: 0.0,
	// 		height: 0.0,
	// 		x: 0.0,
	// 		y: 0.0,
	// 	};
	// 	b.add_line();
	// 	b
	// }

	pub fn add_line(&mut self) {
		self.lines.push(Line::new());
	}

	pub fn get_last_line(&mut self) -> &mut Line<'a> {
		self.lines.last_mut().unwrap()
	}

	pub fn get_prev_line(&mut self) -> &mut Line<'a> {
		let i = self.lines.len();
		&mut self.lines[i-2]
	}
}


pub struct FormatChunkIter<'a> {
	index: usize,
	chunk: &'a FormatChunk,
	sub_iter: Option<Box<FormatChunkIter<'a>>>,
}


impl <'a> Iterator for FormatChunkIter<'a> {
	type Item = (&'a FormatChunk, &'a str);

	fn next (&mut self) -> Option<Self::Item> {
		if self.chunk.chunks.len() == self.index {return None};

		match &self.chunk.chunks[self.index] {
			FormatChunks::String(s) => {
				self.index += 1;
				Some((self.chunk, s))
			}
			FormatChunks::Chunk(_) => {
				if self.sub_iter.is_none() {
					if let FormatChunks::Chunk(c) = &self.chunk.chunks[self.index] {
						let iter = c.iter();
						self.sub_iter = Some(Box::new(iter));
					}
				}
				let data = self.sub_iter.as_mut().unwrap().next();
				if data.is_none() {
					self.index+=1;
					self.next()
				} else {
					data
				}
			}
		}
	}
}