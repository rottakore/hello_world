// Dependencies >>
use std::io::{stdin, stdout, Read, Write};
// << Dependencies

// Pause function >>
fn pause() {
	let mut stdout = stdout();
	stdout
		.write(b"")
		.unwrap();
	stdout
		.flush()
		.unwrap();
	stdin()
		.read(&mut [0])
		.unwrap();
}
// << Pause function

// HelloWorld function >>
fn main() {

	// Message >>
	println!("Hello, world!");
	// << Message

	pause();
}
// << HelloWorld function
