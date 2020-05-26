use tetris::*;

#[allow(dead_code)]
fn print_board(state: &State) {
	for row in state.board.grid.iter() {
		for cell in row.iter() {
			print!("{} ", if *cell { 'X' } else { '.' });
		}
		println!();
	}
}

#[test]
fn drop_one() {
	let state = State::new();

	let mino = Mino::new(MinoShape::J)
		.rotated(2)
		.translated(-2, 0);
	let _state = state.drop(mino).unwrap();
}

// A more comprehensive test:
// Uses each mino at least once in order to perform an FC
#[test]
fn full_clear() {
	let state = State::new();

	let mino = Mino::new(MinoShape::O)
		.rotated(0)
		.translated(-4, 0);
	let state = state.drop(mino).unwrap();

	let mino = Mino::new(MinoShape::L)
		.rotated(1)
		.translated(-3, 0);
	let state = state.drop(mino).unwrap();

	let mino = Mino::new(MinoShape::L)
		.rotated(0)
		.translated(-4, 0);
	let state = state.drop(mino).unwrap();

	let mino = Mino::new(MinoShape::T)
		.rotated(2)
		.translated(-1, 0);
	let state = state.drop(mino).unwrap();

	let mino = Mino::new(MinoShape::T)
		.rotated(-1)
		.translated(-2, 0);
	let state = state.drop(mino).unwrap();

	let mino = Mino::new(MinoShape::Z)
		.rotated(0)
		.translated(1, 0);
	let state = state.drop(mino).unwrap();

	let mino = Mino::new(MinoShape::S)
		.rotated(1)
		.translated(2, 0);
	let state = state.drop(mino).unwrap();

	let mino = Mino::new(MinoShape::Z)
		.rotated(0)
		.translated(0, 0);
	let state = state.drop(mino).unwrap();

	let mino = Mino::new(MinoShape::J)
		.rotated(0)
		.translated(2, 0);
	let state = state.drop(mino).unwrap();

	let mino = Mino::new(MinoShape::I)
		.rotated(1)
		.translated(4, 0);
	let state = state.drop(mino).unwrap();

	assert_eq!(state.score, 1200);
	assert_eq!(state.level, 0);
	assert_eq!(state.lines, 4);
}