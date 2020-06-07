mod protocol;

use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use protocol::*;

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
		let mut buffer = Vec::new();
		stream.read(&mut buffer).unwrap();

		let request = String::from_utf8(buffer).unwrap();

		let (state, mino) = parse_request(&request);

		// ...

		let response = make_response(Vec::new());
	}
}