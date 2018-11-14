use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};
use chunk::*;

pub struct Typer {
	block: FormatBlock,
}

impl Typer {

	pub fn new() -> Self {
		Self {
			block: FormatBlock::new(),
		}
	}


	pub fn parse(&mut self, xml_string: &str) -> Vec<FormatBlock> {

		let file = BufReader::new(xml_string.as_bytes());
		let parser = EventReader::new(file);
		
		let mut blocks: Vec<FormatBlock> = Vec::new();
		let mut level: usize = 0;

		fn get_chunk<'a>(chunk: &'a mut FormatChunk, level:usize) -> Option<&'a mut FormatChunk> {
			if level == 0 {
				return Some(chunk);
			}
			let elem = chunk.chunks
				.iter_mut()
				.filter(|e| if let FormatChunks::Chunk(_) = e {true} else {false})
				.last()
				.unwrap();

			if let FormatChunks::Chunk(chunk) = elem {
				return get_chunk(chunk, level-1);
			}
			None
		}

		for e in parser {

			match e {
				Ok( XmlEvent::StartElement { name, attributes, .. } ) => {
					match &name.local_name[..] {
						"block" => {
							level = 0;
							let mut block = self.block.new_empty();
							for attribute in attributes {
								block.set_attribute(&attribute.name.local_name, &attribute.value);
							}
							blocks.push(block);
						}
						"s" => {
							let block = blocks
								.last_mut()
								.expect("uou mast create <block> for <s>");
							
							let chunk = get_chunk(&mut block.chunk, level)
								.unwrap();
							let mut new_chunk = chunk.new_empty();
							for attribute in attributes {
								new_chunk.set_attribute(&attribute.name.local_name, &attribute.value);
							}
							chunk.chunks.push(FormatChunks::Chunk(new_chunk));
							level += 1;
						}
						_=>{}
					}
				}
				Ok( XmlEvent::EndElement{name, ..} ) => {
					match &name.local_name[..] {
						"block" => {
							level = 0;
						}
						"s" => {
							level -= 1;
						}
						_=>{}
					}
				}
				Ok( XmlEvent::Characters(str_chunks) ) | Ok( XmlEvent::Whitespace(str_chunks) ) => {
					if level > 0 {
						let block = blocks
							.last_mut()
							.expect("text must by in <s>");

						let chunk = get_chunk(&mut block.chunk, level).unwrap();
						chunk.chunks.push(FormatChunks::String(str_chunks));
					}
				}
				Err(e) => {
					println!("Error: {}", e);
					break;
				}
				_ => {}
			}
		}
		
		blocks
	}

}
