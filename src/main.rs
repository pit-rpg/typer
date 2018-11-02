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

use std::fs::{File};
use std::path::{PathBuf};
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

	//  let font_data = include_bytes!("../fonts/wqy-microhei/WenQuanYiMicroHei.ttf");
    // This only succeeds if collection consists of one font
    // let font = Font::from_bytes(font_data as &[u8]).expect("Error constructing Font");
	// let fonts = [font];
	let fonts = vec![("default".to_string(), PathBuf::from("fonts/wqy-microhei/WenQuanYiMicroHei.ttf"))];
	let fonts = TextRenderer::load_fonts(fonts);

	let mut renderer = TextRenderer::new();
	renderer.width = 600;
	// renderer.break_word = true;


	let buffer = renderer.render(chunks, &fonts, 1.0);

	let mut imgbuf = image::RgbaImage::from_vec(buffer.width as u32, buffer.height as u32, buffer.buffer).unwrap();
	imgbuf.save("image_example.png").unwrap();



}



