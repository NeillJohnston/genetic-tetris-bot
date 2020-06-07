use std::cmp::max;
use std::collections::HashSet;
use std::collections::VecDeque;

/// The shape a tetromino can have.
#[derive(Clone, Copy, Debug)]
pub enum MinoShape {
	T,
	J,
	Z,
	O,
	S,
	L,
	I
}

impl MinoShape {
	pub fn n_rotations(&self) -> i32 {
		match self {
			MinoShape::T => 4,
			MinoShape::J => 4,
			MinoShape::Z => 2,
			MinoShape::O => 1,
			MinoShape::S => 2,
			MinoShape::L => 4,
			MinoShape::I => 2
		}
	}
}

/// Mino (short for tetromino), the building blocks of a Tetris game
#[derive(Clone, Copy, Debug)]
pub struct Mino {
	shape: MinoShape,
	rot: i32,
	x: i32,
	y: i32
}

impl Mino {
	/// Create a mino with shape `shape`.
	/// Gives it default spawn rotation and position.
	pub fn new(shape: MinoShape) -> Mino {
		let rot = match shape {
			MinoShape::T => 2,
			MinoShape::J => 3,
			MinoShape::Z => 0,
			MinoShape::O => 0,
			MinoShape::S => 0,
			MinoShape::L => 1,
			MinoShape::I => 1
		};

		Mino {
			shape,
			rot,
			x: 5,
			y: 0
		}
	}

	/// Get the points corresponding to this mino.
	pub fn points(&self) -> [(i32, i32); 4] {
		let deltas = match (self.shape, self.rot) {
			// Thanks to meatfighter.com for the table
			(MinoShape::T, 0) => [(-1, 0), (0, 0), (1, 0), (0, -1)],
			(MinoShape::T, 1) => [(0, -1), (0, 0), (1, 0), (0, 1)],
			(MinoShape::T, 2) => [(-1, 0), (0, 0), (1, 0), (0, 1)],
			(MinoShape::T, 3) => [(0, -1), (-1, 0), (0, 0), (0, 1)],

			(MinoShape::J, 0) => [(0, -1), (0, 0), (-1, 1), (0, 1)],
			(MinoShape::J, 1) => [(-1, -1), (-1, 0), (0, 0), (1, 0)],
			(MinoShape::J, 2) => [(0, -1), (1, -1), (0, 0), (0, 1)],
			(MinoShape::J, 3) => [(-1, 0), (0, 0), (1, 0), (1, 1)],

			(MinoShape::Z, 0) => [(-1, 0), (0, 0), (0, 1), (1, 1)],
			(MinoShape::Z, 1) => [(1, -1), (0, 0), (1, 0), (0, 1)],

			(MinoShape::O, 0) => [(-1, 0), (0, 0), (-1, 1), (0, 1)],

			(MinoShape::S, 0) => [(0, 0), (1, 0), (-1, 1), (0, 1)],
			(MinoShape::S, 1) => [(0, -1), (0, 0), (1, 0), (1, 1)],
			
			(MinoShape::L, 0) => [(0, -1), (0, 0), (0, 1), (1, 1)],
			(MinoShape::L, 1) => [(-1, 0), (0, 0), (1, 0), (-1, 1)],
			(MinoShape::L, 2) => [(-1, -1), (0, -1), (0, 0), (0, 1)],
			(MinoShape::L, 3) => [(1, -1), (-1, 0), (0, 0), (1, 0)],
			
			(MinoShape::I, 0) => [(0, -2), (0, -1), (0, 0), (0, 1)],
			(MinoShape::I, 1) => [(-2, 0), (-1, 0), (0, 0), (1, 0)],

			(s, r) => panic!("Attempted to get points for nonexistent mino rotation ({:?}, {})", s, r)
		};

		// lol no array map :(
		let mut res = [(0, 0); 4];
		for i in 0..4 {
			let (dx, dy) = deltas[i];
			res[i] = (self.x + dx, self.y + dy);
		}

		res
	}

	/// Produce a mino that has been rotated `n` times clockwise.
	pub fn rotated(&self, n: i32) -> Mino {
		let r = self.shape.n_rotations();
		// Have to convert n to a positive number first
		let n = (n % r) + r;

		Mino {
			rot: (self.rot + n) % r,
			..*self
		}
	}

	/// Produce a mino that has been moved `dx` units horizontally and `dy`
	/// units vertically.
	pub fn translated(&self, dx: i32, dy: i32) -> Mino {
		Mino {
			x: self.x + dx,
			y: self.y + dy,
			..*self
		}
	}
}

/// Standard 20x10 playing field.
#[derive(Clone, Copy)]
pub struct Board {
	pub grid: [[bool; 10]; 20]
}

impl Board {
	fn blank() -> Board {
		Board {
			grid: [[false; 10]; 20]
		}
	}

	fn can_place(&self, mino: Mino) -> bool {
		mino.points().iter()
			.all(|(x, y)| {
				let (x, y) = (*x, *y);
				// The x-coord must be in bounds, but points may be above the grid
				(0 <= x && x < 10) && (y < 20) && (y < 0 || !self.grid[y as usize][x as usize])
			})
	}
}

#[cfg(test)]
mod board_tests {
}

/// Slice of the game frozen in time, and the primary interface through which
/// games can be simulated.
/// 
/// Has implementations for different methods of accessing/viewing the board
/// data which should aid in writing bots.
#[derive(Clone, Copy)]
pub struct State {
	pub board: Board,
	pub score: i32,
	pub level: i32,
	pub lines: i32
}

impl State {
	/// Construct a brand new state - blank board, level 0.
	pub fn new() -> State {
		State {
			board: Board::blank(),
			score: 0,
			level: 0,
			lines: 0
		}
	}

	/// Construct a brand new state starting at a certain level.
	pub fn with_start(level: i32) -> State {
		State {
			board: Board::blank(),
			score: 0,
			level,
			lines: 0
		}
	}

	/// Produce a new state by placing `mino` on the board.
	/// Clears lines and updates score/level/lines.
	/// 
	/// The resulting state is wrapped in `Option` because `mino` may be
	/// unplaceable.
	pub fn place(&self, mino: Mino) -> Option<State> {
		if !self.board.can_place(mino) {
			return None;
		}

		let mut board = self.board;
		let mut modified = HashSet::new();
		for (x, y) in mino.points().iter() {
			let (x, y) = (*x, *y);
			if y >= 0 {
				// Already called `can_place` so we won't go OOB here
				board.grid[y as usize][x as usize] = true;
				modified.insert(y);
			}
		}

		let mut cleared = HashSet::new();
		for y in modified.into_iter() {
			if board.grid[y as usize].iter().all(|b| *b == true) {
				cleared.insert(y);
			}
		}

		let mut d = 0;
		for y in (0..20).rev() {
			while cleared.contains(&(y - d)) {
				d += 1;
			}

			if y - d < 0 {
				board.grid[y as usize] = [false; 10];
			}
			else {
				board.grid[y as usize] = board.grid[(y - d) as usize];
			}
		}

		let n_cleared = cleared.len() as i32;
		let score = self.score + match n_cleared {
			0 => 0,
			1 => 40*(self.level+1),
			2 => 100*(self.level+1),
			3 => 300*(self.level+1),
			4 => 1200*(self.level+1),
			n => panic!("Cleared too many ({}) lines in one turn", n)
		};
		let lines = self.lines + n_cleared;
		let level = max(self.level, lines/10);

		Some(State {
			board,
			score,
			level,
			lines
		})
	}

	/// Produce a new state by dropping `mino` onto the board.
	pub fn drop(&self, mino: Mino) -> Option<State> {
		let mut mino = mino;
		while self.board.can_place(mino) {
			mino = mino.translated(0, 1);
		}
		mino = mino.translated(0, -1);

		self.place(mino)
	}

	/// Get row `y` from the board.
	pub fn row(&self, y: usize) -> [bool; 10] {
		self.board.grid[y]
	}

	/// Get column `x` from the board.
	pub fn column(&self, x: usize) -> [bool; 20] {
		let mut col = [false; 20];

		for y in 0..20 {
			if self.board.grid[y][x] {
				col[y] = true;
			}
		}

		col
	}

	/// Get cell (x, y) from the board.
	pub fn cell(&self, x: usize, y: usize) -> bool {
		self.board.grid[y][x]
	}

	/// Get the "depth" (number of consecutive empty cells from the top) of
	/// column `x`.
	pub fn column_depth(&self, x: usize) -> usize {
		let col = self.column(x);
		for (y, cell) in col.iter().enumerate() {
			if *cell {
				return y;
			}
		}

		20
	}

	/// Get all possible future board states and the minos that cause them.
	pub fn possibilities(&self, next: MinoShape) -> Vec<(State, Mino)> {
		// Algorithm is a simple BFS, moving minos one unit/rotation at a time

		// Visited state for minos - visited[x][y][rot]
		let mut visited = [[[false; 4]; 20]; 10];

		let mut queue = VecDeque::new();
		queue.push_back(Mino::new(next));

		let mut minos = Vec::new();

		while !queue.is_empty() {
			let Mino { x, y, rot, .. } = queue.pop_front().unwrap();
			let mino = Mino { x, y, rot, shape: next };

			if !self.board.can_place(mino) {
				continue;
			}

			let (xi, yi, roti) = (x as usize, y as usize, rot as usize);
			if visited[xi][yi][roti] {
				continue;
			}
			visited[xi][yi][roti] = true;

			queue.push_back(mino.translated(0, 1));
			queue.push_back(mino.translated(-1, 0));
			queue.push_back(mino.translated(1, 0));
			queue.push_back(mino.rotated(-1));
			queue.push_back(mino.rotated(1));

			let down = mino.translated(0, 1);
			if !self.board.can_place(down) {
				minos.push(mino);
			}
		}

		minos.into_iter()
			.map(|mino| (self.place(mino).unwrap(), mino))
			.collect()
	}
}

#[cfg(test)]
mod state_tests {
	use crate::*;

	#[test]
	fn possibilities() {
		let state = State::new();
		let next = MinoShape::J;
		let possibilities = state.possibilities(next);
		assert_eq!(possibilities.len(), 34);

		let state = State::new();
		let mino = Mino::new(MinoShape::S).translated(0, 18);
		let state = state.place(mino).unwrap();
		let next = MinoShape::J;
		let possibilities = state.possibilities(next);
		// 35 possibilities now that there's a tuck
		assert_eq!(possibilities.len(), 35);
	}
}
