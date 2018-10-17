use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};
use state::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Tags {
	S,
	B,
	I,
}

impl Tags {

	pub fn new (tag: &str) -> Self {
		// let state = State::new();
		match tag {
			"b" 	=> {Tags::B}
			"i" 	=> {Tags::I}
			_ 		=> {Tags::S}
		}
	}

	pub fn get_state(&self) -> State {
		let mut state = State::new();
		match self {
			Tags::B => state.bold = StateBool::True,
			Tags::I => state.italic = StateBool::True,
			_ => {},
		}
		state
	}
}


pub struct Typer {
	pub state: State,
}

impl Typer {
	pub fn new() -> Self {
		Self {
			state: State::default(),
		}
	}

	pub fn parse(&mut self, xml_string: &str) {

		let file = BufReader::new(xml_string.as_bytes());
		let parser = EventReader::new(file);

		let mut current_state 					= self.state.clone();
		let mut states: Vec<State> 				= Vec::new();
		let mut chunks: Vec<(State, String)> 	= Vec::new();

		for e in parser {
			match e {
				Ok( XmlEvent::StartElement { name, attributes, .. } ) => {
					let tag = Tags::new(&name.local_name);
					let mut state = tag.get_state();
					for attribute in attributes {
						state.set_attribute(&attribute.name.local_name, &attribute.value);
					}
					state = current_state.clone() + state;
					states.push(current_state);
					current_state = state;
				}
				Ok( XmlEvent::EndElement{..} ) => {
					// println!("pop");
					current_state = states.pop().unwrap();
					// println!("{:?}", current_state);
				}
				Ok( XmlEvent::Characters(str_chunks) ) => {
					chunks.push( (current_state.clone(), str_chunks) )
				}
				Err(e) => {
					println!("Error: {}", e);
					break;
				}
				_ => {}
			}
		}

		println!("{:?}", chunks);
		println!("{:?}", states);
	}
}