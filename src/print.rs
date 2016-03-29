use ansi_term::Colour::{Red, Green};		//Blue, Yellow, 


pub fn println_red(str: String) {
	println!("{}", Red.paint(str));
}

pub fn println_green(str: String) {
	println!("{}", Green.paint(str));
}
