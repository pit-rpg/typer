extern crate xml;
extern crate rusttype;
extern crate image;

mod chunk;
mod typer;
mod units;
mod rusttype_renderer;

use chunk::*;
use typer::*;
use units::*;
use rusttype_renderer::*;

use std::fs::{File};
use std::path::{PathBuf};
use std::io::Read;

use self::rusttype::{Font};


fn main() {

	let mut typer = Typer::new();

    let mut file = File::open("file.xml").unwrap();
	let mut data = String::new();
	file.read_to_string(&mut data).unwrap();

	let blocks = typer.parse(&data);

	let fonts = vec![("default".to_string(), PathBuf::from("fonts/wqy-microhei/WenQuanYiMicroHei.ttf"))];
	let fonts = TextRenderer::load_fonts(fonts);


	let mut layout = TextRenderer::format(blocks, 1.0, &fonts);
	layout.calk_view();
	layout.width = 600.0;
	layout.height = 600.0;

	layout.x = 0.0;
	// layout.x = 100.0;
	layout.y = 0.0;

	// layout.x = 200.0;

	let mut buffer = layout.create_buffer().unwrap();
	TextRenderer::render(&layout, &mut buffer);

	println!("================ RENDERED ================");
	println!("layout {}x{}", layout.width, layout.height);
	println!("buffer {}x{}", buffer.width, buffer.height);
	let img_buf = image::RgbaImage::from_vec(buffer.width as u32, buffer.height as u32, buffer.buffer).unwrap();
	img_buf.save("image_example.png").unwrap();



}



