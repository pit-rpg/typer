
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct ColorRGBA {
	pub r:u8,
	pub g:u8,
	pub b:u8,
	pub a:u8,
}


impl Default for ColorRGBA {
	fn default() -> Self {
		Self {r:0, g:0, b:0, a:255}
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
	x:f32,
	y:f32,
}