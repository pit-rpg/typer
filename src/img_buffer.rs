
pub type ColorRGBA = [u8;4];

pub struct ImgBuffer {
	pub buffer: Vec<u8>,
	pub width: usize,
	pub height: usize,
}


pub struct ImgBufferRef<'a> {
	pub buffer: &'a mut Vec<u8>,
	pub width: usize,
	pub height: usize,
}

pub trait ImgBufferTrait {
	#[inline] fn get_pixel_mut(&mut self, x: usize, y: usize) -> &mut [u8];
	#[inline] fn width(&self) -> usize;
	#[inline] fn height(&self) -> usize;
	#[inline] fn get_buffer_mut(&mut self) -> &mut Vec<u8>;

	fn blend_pixel (&mut self, x:usize ,y:usize, pixel: &[u8;4], v:f32) {
		if 
			x > self.width()-1 || 
			y > self.height()-1
				{return}

		let o_pixel = self.get_pixel_mut(x, y);
		let f_pixel = [
			pixel[0] as f32 / 255.0,
			pixel[1] as f32 / 255.0,
			pixel[2] as f32 / 255.0,
			pixel[3] as f32 / 255.0,
		];
		let alpha = f_pixel[3] * v;
		let o_alpha = o_pixel[3] as f32 / 255.0;

		o_pixel[0] = ((((o_pixel[0] as f32 / 255.0) * (1.0-alpha)) + (f_pixel[0]*alpha)) * 255.0) as u8;
		o_pixel[1] = ((((o_pixel[1] as f32 / 255.0) * (1.0-alpha)) + (f_pixel[1]*alpha)) * 255.0) as u8;
		o_pixel[2] = ((((o_pixel[2] as f32 / 255.0) * (1.0-alpha)) + (f_pixel[2]*alpha)) * 255.0) as u8;
		o_pixel[3] = ( (alpha * v).max(o_alpha) * 255.0 ) as u8;
	}

	fn put_pixel(&mut self, x:usize ,y:usize, pixel: &[u8;4]) {
		let o_pixel = self.get_pixel_mut(x, y);
		o_pixel[0] = pixel[0];
		o_pixel[1] = pixel[1];
		o_pixel[2] = pixel[2];
		o_pixel[3] = pixel[3];
	}

	fn clear(&mut self, color: &[u8;4]) {
		let buf = self.get_buffer_mut();
		for i in 0..buf.len() {
			buf[i] = color[i%4];
		}
	}

	fn fill(&mut self, n: u8) {
		let buf = self.get_buffer_mut();
		buf
			.iter_mut()
			.for_each(|e| *e = n )
	}
}

impl ImgBuffer {
	pub fn new(width:usize, height:usize, fill: &[u8; 4]) -> Self {
		let capacity = width * height * 4;
		let mut buffer: Vec<u8> = Vec::with_capacity(capacity);

		for _ in 0..width * height {
			buffer.extend_from_slice(fill);
		}

		Self {
			width,
			height,
			buffer,
		}
	}
}

impl ImgBufferTrait for ImgBuffer {

	#[inline] fn width(&self) -> usize {self.width}
	#[inline] fn height(&self) -> usize {self.height}
	#[inline] fn get_pixel_mut(&mut self, x: usize, y: usize) -> &mut [u8] {
		let i =  y * (self.width*4) + (x * 4);
		&mut self.buffer[i..(i+4)]
	}
	#[inline] fn get_buffer_mut(&mut self) -> &mut Vec<u8> {
		&mut self.buffer
	}

}


impl <'a> ImgBufferRef<'a> {
	pub fn new(width:usize, height:usize, buffer: &'a mut Vec<u8>) -> Self {
		Self {
			width,
			height,
			buffer,
		}
	}	
}


impl <'a> ImgBufferTrait for ImgBufferRef<'a> {
	
	#[inline] fn width(&self) -> usize {self.width}
	#[inline] fn height(&self) -> usize {self.height}
	#[inline] fn get_pixel_mut(&mut self, x: usize, y: usize) -> &mut [u8] {
		let i =  y * (self.width*4) + (x * 4);
		&mut self.buffer[i..(i+4)]
	}
	#[inline] fn get_buffer_mut(&mut self) -> &mut Vec<u8> {
		&mut self.buffer
	}
}
