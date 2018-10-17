


extern crate xml;

mod state;
mod typer;

use state::*;
use typer::*;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;

use xml::reader::{EventReader, XmlEvent};









fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

fn main() {

	let mut typer = Typer::new();

    let mut file = File::open("file.xml").unwrap();
	let mut data = String::new();
	file.read_to_string(&mut data).unwrap();

	typer.parse(&data);


    // let file = BufReader::new(file);

    // let parser = EventReader::new(file);
    // let mut depth = 0;
    // for e in parser {
    //     match &e {
    //         Ok(XmlEvent::StartElement { name, .. }) => {
    //             println!("{}+{}", indent(depth), name);
    //             // println!("{}+{:?}", indent(depth), e);
    //             depth += 1;
    //         }
    //         Ok(XmlEvent::EndElement { name }) => {
    //             depth -= 1;
    //             println!("{}-{}", indent(depth), name);
    //         }
    //         Ok(XmlEvent::Characters(eeee)) => {
    //             // depth -= 1;
    //             println!("{} {}", indent(depth), eeee);
    //         }
    //         Err(e) => {
    //             println!("Error: {}", e);
    //             break;
    //         }
    //         _ => {}
    //     }
    //     // println!("{:?}", e);
    // }
}


// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }



