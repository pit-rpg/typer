use std::ops::Add;
use super::units::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TextAlignHorizontal {
	Left,
	Right,
	Center,
}


#[derive(Debug, Clone)]
pub struct Chunk {
	pub bold: Option<bool>,
	pub italic: Option<bool>,
	pub font_size: Option<usize>,
	pub color: Option<ColorRGBA>,
	pub text_align_horizontal: Option<TextAlignHorizontal>,
	pub font: Option<String>,
	pub string: String,
}



impl Chunk {
	pub fn new() -> Self {
		Self {
			bold: None,
			italic: None,
			font_size: None,
			color: None,
			text_align_horizontal: None,
			font: None,
			string: "".to_string(),
		}
	}

	pub fn set_attribute(&mut self, key: &str, val: &str) {
		match key {
			"font_size" 	=> { self.font_size = Some(val.parse::<usize>().unwrap()) }
			"font" 			=> { self.font = Some(val.to_string()) }
			"bold" 			=> {
				match val {
					"true"|"TRUE"|"1"|"yes" 	=> { self.bold = Some(true) }
					"false"|"FALSE"|"0"|"no" 	=> { self.bold = Some(false) }
					_ => {}
				}
			}
			"italic" 			=> {
				match val {
					"true"|"TRUE"|"1"|"yes" 	=> { self.italic = Some(true) }
					"false"|"FALSE"|"0"|"no" 	=> { self.italic = Some(false) }
					_ => {}
				}
			}
			_ => {
				println!("unknown attribute: {}", key);
			}
		}
	}

	pub fn patch(&mut self, other: &Self) -> &mut Self {

		if other.font_size != None 	{self.font_size = other.font_size;}
		if other.bold != None 		{self.bold = other.bold;}
		if other.italic != None 	{self.italic = other.italic;}
		if other.color != None 		{self.color = other.color;}
		if other.font != None 		{self.font = other.font.clone();}

		self
	}

	pub fn duplicate(&self) -> Self {
		let mut  n = Self::new();
		n.patch(self);
		n
	}
}


impl Default for Chunk {
	fn default() -> Self {
		Self {
			bold: Some(false),
			italic: Some(false),
			font_size: Some(10),
			color: Some( [0.0, 0.0, 0.0, 0.5] ),
			text_align_horizontal: Some(TextAlignHorizontal::Left),
			font: None,
			string: "".to_string(),
		}
	}
}