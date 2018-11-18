extern crate image;
extern crate typer;

use std::fs::{File};
use std::path::{PathBuf, Path};
use std::io::Read;
use typer::{TextRenderer, Typer, ImgBufferRef, ImgBufferTrait};
use image::*;

fn main() {
	let im = image::open(&Path::new("assets/predator.jpg")).unwrap().to_rgba();

	let mut file = File::open("examples/example-3.xml").unwrap();
	let mut data = String::new();
	file.read_to_string(&mut data).unwrap();
	let fonts = vec![
		("__".to_string(), PathBuf::from("fonts/Roboto-Regular.ttf"))
	];

	let fonts = TextRenderer::load_fonts(fonts);
	let mut typer = Typer::new();
	let blocks = typer.parse(&data);
	let mut layout = TextRenderer::format(blocks, 1.0, &fonts);
	
	layout.calk_view();
	let borders = 20;
	let t_box = (layout.x.floor() as isize, layout.y.floor() as isize, layout.width.ceil() as isize, layout.height.ceil() as isize);
	layout.x = 0.0;
	layout.y = 0.0;
	
	let width = im.width();
	let height = im.height();
	let mut buf = im.into_raw();
	{
		let mut buffer = ImgBufferRef::new(width as usize, height as usize, &mut buf);

		for x in t_box.0-borders..t_box.2+borders {
			for y in t_box.1-borders..t_box.3+borders {
				buffer.blend_pixel(x.max(0) as usize, y.max(0) as usize, &[0,0,0,255], 0.8);
			}			
		}

		TextRenderer::render(&layout, &mut buffer);
	}

	let img_buf = image::RgbaImage::from_vec(width as u32, height as u32, buf).unwrap();
	img_buf.save("examples/out.png").unwrap();


	println!("RENDERED: examples/out.png");
	println!("img {}x{}", width, height);
}
