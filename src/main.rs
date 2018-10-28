extern crate xml;
extern crate rusttype;

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

	let renderer = TextRenderer::new();
	renderer.render(chunks, &[font], 1.0);
}


// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }



