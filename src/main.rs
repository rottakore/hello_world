// Dependencies >>
use std::io::{stdin, Read};
// << Dependencies

// "hello_world" function >>
fn main() {

	// Message >>
	println!("Hello, world!");
	// << Message

	// Pausing the program until user hits enter >>
	stdin()
		.read(&mut [0])
		.unwrap();
	// << Pausing the program until user hits enter
}
// << "hello_world" function
