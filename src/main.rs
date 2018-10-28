extern crate xml;
extern crate rusttype;
extern crate image;

mod chunk;
mod typer;
mod units;
mod renderer;

use chunk::*;
use typer::*;
use units::*;
use renderer::*;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use image::{RgbaImage, Rgba};

use xml::reader::{EventReader, XmlEvent};
use self::rusttype::{Font};


fn main() {

	let mut typer = Typer::new();

    let mut file = File::open("file.xml").unwrap();
	let mut data = String::new();
	file.read_to_string(&mut data).unwrap();

	let chunks = typer.parse(&data);

	 let font_data = include_bytes!("../fonts/wqy-microhei/WenQuanYiMicroHei.ttf");
    // This only succeeds if collection consists of one font
    let font = Font::from_bytes(font_data as &[u8]).expect("Error constructing Font");
	let fonts = [font];

	let mut renderer = TextRenderer::new();

	let buffer = renderer.render(chunks, &fonts, 1.0);

	println!("w:{}, h:{}", buffer.width, buffer.height);
	println!("len: {}, c:{}", buffer.buffer.len(), buffer.width * buffer.height * 4);
	// let buffer.buffer;
	// for i in 0..buffer.height {
	// 	let height = buffer.height as usize;
	// 	let width = buffer.width as usize;
	// 	let slice = &buffer.buffer[ width*i*4..width*(i+1)*4 ];
	// 	// let slice: Vec<u8> = slice
	// 	// 	.iter()
	// 	// 	.step_by(4)
	// 	// 	.map( |e| if *e > 100 {1} else {0} )
	// 	// 	.collect();

	// 	println!("{:?}", slice);
	// }

	// let mut pixels = Vec::with_capacity(buffer.width * buffer.height);
	// for i in 0 .. buffer.width * buffer.height {
	// 	let pixel = Rgba { data:[
	// 		buffer.buffer[i  ],
	// 		buffer.buffer[i+1],
	// 		buffer.buffer[i+2],
	// 		buffer.buffer[i+3]
	// 	]};
	// 	pixels.push(pixel);
	// }
	let mut imgbuf = image::RgbaImage::from_vec(buffer.width as u32, buffer.height as u32, buffer.buffer).unwrap();
	imgbuf.save("image_example.png").unwrap();


	// let mut imgbuf = image::RgbaImage::new(buffer.width as u32, buffer.height as u32);
	// let mut imgbuf = image::GrayImage::new(buffer.width, buffer.height);


}


// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }



