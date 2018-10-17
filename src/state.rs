use std::ops::Add;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StateBool {
	True,
	False,
	None,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StateFontSize {
	Size(usize),
	None,
}


#[derive(Debug, Clone)]
pub struct State {
	pub bold: StateBool,
	pub italic: StateBool,
	pub font_size: StateFontSize,
}

impl State {
	pub fn new() -> Self {
		Self {
			bold: StateBool::None,
			italic: StateBool::None,
			font_size: StateFontSize::None,
		}
	}

	pub fn set_attribute(&mut self, key: &str, val: &str) {
		println!("{} - {}", key, val);
		match key {
			"font_size" 	=> { self.font_size = StateFontSize::Size( val.parse::<usize>().unwrap()  ) }
			"bold" 			=> {
				match val {
					"true"|"TRUE"|"1"|"yes" 	=> { self.bold = StateBool::True }
					"false"|"FALSE"|"0"|"no" 	=> { self.bold = StateBool::False }
					_ => {}
				}
			}
			"italic" 			=> {
				match val {
					"true"|"TRUE"|"1"|"yes" 	=> { self.italic = StateBool::True }
					"false"|"FALSE"|"0"|"no" 	=> { self.italic = StateBool::False }
					_ => {}
				}
			}
			_ => {
				println!("unknown attribute: {}", key);
			}
		}
	}
}

impl Default for State {
	fn default() -> Self {
		Self {
			bold: StateBool::False,
			italic: StateBool::False,
			font_size: StateFontSize::Size(10),
		}
	}
}

impl Add for State {
	type Output = State;

	fn add(self, b: Self) -> Self {
		let mut res = self.clone();

		if b.font_size != StateFontSize::None 	{res.font_size = b.font_size;}
		if b.bold != StateBool::None 			{res.bold = b.bold;}
		if b.italic != StateBool::None 			{res.italic = b.italic;}
		res
	}
}