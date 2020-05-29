use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;

fn main() {
	let listener = TcpListener::bind("127.0.0.1:10000").unwrap();

	for stream in listener.incoming() {
		match stream {
			Ok(stream) => handle(stream),
			Err(_) => { println!("err") }
		};
	}
}

fn handle(mut stream: TcpStream) {
	println!("received a connection");

	loop {
		let mut buffer = [0; 1024];
		stream.read(&mut buffer).unwrap();

		let message = String::from_utf8_lossy(&buffer[..]);
		println!("{}", message);
	}
}