use tetris::*;
use std::io::{Write};

/// # Tetris Decision Server Protocol
/// (Writing this down so I don't screw up)
/// 
/// ## Requests
/// The first 20 lines specify the state of the Tetris grid. Each line is to
/// contain exactly 10 characters, either '.' (denoting empty space) or 'x'
/// (denoting a block).
/// The next line specifies the current and next piece information, as a space
/// separated list of uppercase characters.
/// The next (and last) line contains three integers, in order: the current
/// level, the current score, and the current number of lines cleared.
/// 
/// ### Example:
/// ```
/// ..........
/// ..........
/// ..........
/// ..........
/// ..........
/// ..........
/// ..........
/// ..........
/// ..........
/// ..........
/// ..........
/// ..........
/// ..........
/// x.........
/// xxx...xxx.
/// xxxxxxxxx.
/// xxxxxxxxx.
/// xxxxxxxxx.
/// xxxxxxxxx.
/// xxxxxxxxx.
/// Z I
/// 18 22800 4
/// ```
/// 
/// ## Responses
/// Responses are lists of (x, y, r) triples - each one specifies a point and
/// rotation that the player must navigate to in order to be set up for the
/// next part of the move. `x` and `y` are absolute grid coordinates, and `r`
/// is the number of times the player must rotate clockwise. Negative values
/// for `r` are permitted, e.g. `r` = -1 means rotate once counterclockwise.
/// 
/// The first line contains a single integer `n` denoting how many steps there
/// are to this move.
/// The next `n` lines contain three integers each - the (x, y, r) triples
/// described above.
/// 
/// ### Example
/// A T-spin - the T-piece moves to the right a little bit and rotates CCW,
/// then drops 4 more spaces, then rotates CW to complete the spin.
/// ```
/// 3
/// 7 10 -1
/// 7 14 0
/// 7 14 1
/// ```

/// Turn a request string into accompanying state and next piece.
pub fn parse_request(request: &String) -> (State, MinoShape) {
	let lines: Vec<&str> = request.lines().collect();

	let mut state = State::new();
	for y in 0..20 {
		for (x, cell) in lines[y].chars().enumerate() {
			state.board.grid[y][x] = match cell {
				'.' => false,
				'x' => true,
				_ => { panic!("Unparseable character at ({}, {}) in board of request", x, y); }
			}
		}
	}

	let mino = match lines[20].chars().nth(0) {
		Some('I') => MinoShape::I,
		Some('J') => MinoShape::J,
		Some('L') => MinoShape::L,
		Some('O') => MinoShape::O,
		Some('S') => MinoShape::S,
		Some('T') => MinoShape::T,
		Some('Z') => MinoShape::Z,
		Some(x) => { panic!("Unparseable mino type {} in request", x); },
		None => { panic!("No next piece given in request"); }
	};

	(state, mino)
}

/// Turn a path into a response string.
pub fn make_response(path: Vec<(i32, i32, i32)>) -> String {
	let mut response = Vec::new();
	writeln!(&mut response, "{}", path.len()).unwrap();
	for (x, y, r) in path.iter() {
		writeln!(&mut response, "{} {} {}", x, y, r).unwrap();
	}

	String::from_utf8(response).unwrap()
}