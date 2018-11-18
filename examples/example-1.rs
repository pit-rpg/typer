extern crate image;
extern crate typer;

use std::fs::{File};
use std::path::{PathBuf};
use std::io::Read;
use typer::{TextRenderer, Typer};

fn main() {

	let mut typer = Typer::new();

	let mut file = File::open("examples/example-1.xml").unwrap();
	let mut data = String::new();
	file.read_to_string(&mut data).unwrap();

	let fonts = vec![
		("default".to_string(), PathBuf::from("fonts/wqy-microhei/WenQuanYiMicroHei.ttf")),
		("opensans-italic".to_string(), PathBuf::from("fonts/opensans/OpenSans-Italic.ttf")),
		("dejavu".to_string(), PathBuf::from("fonts/dejavu/DejaVuSansMono.ttf")),
		("roboto".to_string(), PathBuf::from("fonts/Roboto-Regular.ttf"))
	];
	let fonts = TextRenderer::load_fonts(fonts);

	let blocks = typer.parse(&data);
	let mut layout = TextRenderer::format(blocks, 1.0, &fonts);
	layout.calk_view();

	let mut buffer = layout.create_buffer(&[255,255,255,255]).unwrap();
	TextRenderer::render(&layout, &mut buffer);

	let img_buf = image::RgbaImage::from_vec(buffer.width as u32, buffer.height as u32, buffer.buffer).unwrap();
	img_buf.save("examples/out.png").unwrap();
	println!("RENDERED: examples/out.png");
	println!("img {}x{}", buffer.width, buffer.height);
}
