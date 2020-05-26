use tetris::*;
use rand::random;

/// A simulatable Tetris bot.
pub trait Bot {
	/// Take a state and outputs a score based on how desirable that state is
	/// (higher = better).
	fn evaluate(&self, state: State) -> f64;
}

fn random_mino() -> MinoShape {
	// Not uniformly distributed but it's definitely close enough
    match (random::<u64>()) % 7 {
        0 => MinoShape::I,
        1 => MinoShape::J,
        2 => MinoShape::L,
        3 => MinoShape::O,
        4 => MinoShape::S,
        5 => MinoShape::T,
        6 => MinoShape::Z,
        _ => MinoShape::O
    }
}

// Run a single turn in the game. Finds and feeds possible future states to
// the bot, and returns the one that evaluates highest.
fn turn<T: Bot>(state: State, next: MinoShape, bot: &T) -> Option<State> {
    let mino = Mino::new(next);

    let possibilities = (0..next.n_rotations())
        .map(|n| state.all_drops(mino.rotated(n)))
        .flatten();

    // Have to do this because `f64` does not have `cmp`
    possibilities.max_by(|a: &State, b: &State| {
        if bot.evaluate(*a) < bot.evaluate(*b) {
            std::cmp::Ordering::Less
        }
        else {
            std::cmp::Ordering::Greater
        }
    })
}

/// Simulate `n` games played by `bot` and return the average score.
pub fn simulate<T: Bot>(n: u32, bot: &T) -> f64 {
    let mut sum = 0.0;

    for _ in 0..n {
		let mut state = State::new();
		// Simulated kill-screen at 500 lines
        while state.lines < 500 {
            let next = random_mino();
            state = match turn(state, next, bot) {
                Some(state) => state,
                None => { break; }
            };
        }

        sum += state.score as f64;
    }

    sum / (n as f64)
}