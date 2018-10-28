
// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub type ColorRGBA = [f32;4];

pub struct ImgBuffer {
	pub buffer: Vec<u8>,
	pub width: usize,
	pub height: usize,
}

impl ImgBuffer {
	pub fn new(width:usize, height:usize, fill:&[f32;4]) -> Self {
		let capacity = width * height * 4;
		let mut buffer: Vec<u8> = Vec::with_capacity(capacity);
		let f = [
			(fill[0] * 255.0) as u8,
			(fill[1] * 255.0) as u8,
			(fill[2] * 255.0) as u8,
			(fill[3] * 255.0) as u8,
		];
		for _ in 0..width * height {
			buffer.extend_from_slice(&f);
		}

		Self {
			width,
			height,
			buffer,
		}
	}

	pub fn put_pixel(&mut self, x:usize ,y:usize, pixel:[u8;4]) {
		// println!("x:{:?}, self.width:{}, y: {}", x, self.width,y);
		let i = (x*4) + (self.width*4 * y);
		self.buffer[ i   ] = pixel[0];
		self.buffer[ i+1 ] = pixel[1];
		self.buffer[ i+2 ] = pixel[2];
		self.buffer[ i+3 ] = pixel[3];
	}
}

// pub struct ColorRGBA {
// 	pub r:u8,
// 	pub g:u8,
// 	pub b:u8,
// 	pub a:u8,
// }


// impl Default for ColorRGBA {
// 	fn default() -> Self {
// 		ColorRGBA([0,0,0,255])
// 	}
// }

// impl ColorRGBA {
// 	pub fn new(r:u8,g:u8,b:u8,a:u8) -> Self {
// 		ColorRGBA([r,g,b,a])
// 	}
// }

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
	x:f32,
	y:f32,
}