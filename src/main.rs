// Dependencies >>
use std::io::{stdin, stdout, Read, Write};
// << Dependencies

// "pause" function >>
fn pause() {
	//let mut stdout = stdout();
	stdout()
		.write(b"")
		.unwrap();
	stdout()
		.flush()
		.unwrap();
	stdin()
		.read(&mut [0])
		.unwrap();
}
// << "pause" function

// "hello_world" function >>
fn main() {

	// Message >>
	println!("Hello, world!");
	// << Message

	// Evoking "pause" function >>
	pause();
	// << Evoking "pause" function
}
// << "hello_world" function
