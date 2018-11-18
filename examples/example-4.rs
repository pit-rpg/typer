extern crate image;
extern crate typer;

use std::fs::{File};
use std::path::{PathBuf, Path};
use std::io::Read;
use typer::{TextRenderer, Typer, ImgBufferRef};
use image::*;

fn main() {
	let im = image::open(&Path::new("assets/alienvspredator.jpg")).unwrap().to_rgba();

	let mut file = File::open("examples/example-4.xml").unwrap();
	let mut data = String::new();
	file.read_to_string(&mut data).unwrap();
	let fonts = vec![
		("__".to_string(), PathBuf::from("fonts/Roboto-Regular.ttf"))
	];

	let fonts = TextRenderer::load_fonts(fonts);
	let mut typer = Typer::new();
	let blocks = typer.parse(&data);

	let layout = TextRenderer::format(blocks, 1.0, &fonts);
		
	let width = im.width();
	let height = im.height();
	let mut buf = im.into_raw();
	{
		let mut buffer = ImgBufferRef::new(width as usize, height as usize, &mut buf);
		TextRenderer::render(&layout, &mut buffer);
	}

	let img_buf = image::RgbaImage::from_vec(width as u32, height as u32, buf).unwrap();
	img_buf.save("examples/out.png").unwrap();

	println!("RENDERED: examples/out.png");
	println!("img {}x{}", width, height);
}
