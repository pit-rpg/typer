use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};
use chunk::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Tags {
	S,
	B,
	I,
}

impl Tags {

	pub fn new (tag: &str) -> Self {
		// let chunk = Chunk::new();
		match tag {
			"b" 	=> {Tags::B}
			"i" 	=> {Tags::I}
			_ 		=> {Tags::S}
		}
	}

	pub fn cet_chunk(&self) -> Chunk {
		let mut chunk = Chunk::new();
		match self {
			Tags::B => chunk.bold = Some(true),
			Tags::I => chunk.italic = Some(true),
			_ => {},
		}
		chunk
	}
}


pub struct Typer {
	pub chunk: Chunk,
}

impl Typer {

	pub fn new() -> Self {
		Self {
			chunk: Chunk::default(),
		}
	}


	pub fn parse(&mut self, xml_string: &str) -> Vec<Chunk> {

		let file = BufReader::new(xml_string.as_bytes());
		let parser = EventReader::new(file);

		let mut current_chunk 					= self.chunk.clone();
		let mut chunks_stack: Vec<Chunk> 				= Vec::new();
		let mut chunks: Vec<Chunk> 	= Vec::new();

		for e in parser {
			match e {
				Ok( XmlEvent::StartElement { name, attributes, .. } ) => {
					let tag = Tags::new(&name.local_name);
					let mut chunk = tag.cet_chunk();
					for attribute in attributes {
						chunk.set_attribute(&attribute.name.local_name, &attribute.value);
					}
					let mut new_chunk = Chunk::new();
					new_chunk.patch(&current_chunk).patch(&chunk);
					chunks_stack.push(current_chunk);
					current_chunk = new_chunk;
				}
				Ok( XmlEvent::EndElement{..} ) => {
					current_chunk = chunks_stack.pop().unwrap();
				}
				Ok( XmlEvent::Characters(str_chunks) ) => {
					let mut new_chunk = Chunk::new();
					new_chunk.patch(&current_chunk);
					new_chunk.string = str_chunks;
					chunks.push( new_chunk );
				}
				Err(e) => {
					println!("Error: {}", e);
					break;
				}
				_ => {}
			}
		}

		println!("{:?}", chunks);
		println!("{:?}", chunks_stack);
		chunks
	}

}
