use std::io::{stdin, stdout, Read, Write};

fn pause() {
	let mut stdout = stdout();
	stdout.write(b"").unwrap();
	stdout.flush().unwrap();
	stdin().read(&mut [0]).unwrap();
}

fn main() {
    println!("Hello, world!");
	pause();
}
