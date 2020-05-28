#[path = "util.rs"]
mod util;

use tetris::*;
use rand::random;
use util::f64_cmp;

/// A simulatable Tetris bot.
pub trait Bot {
	/// Take a state and outputs a score based on how desirable that state is
	/// (higher = better).
	fn evaluate(&self, state: &State) -> f64;
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

/// Run a single turn in the game. Finds and feeds possible future states to
/// the bot, and returns the one that evaluates highest + the mino placed that
/// got it there.
fn turn<T: Bot>(state: &State, next: MinoShape, bot: &T) -> Option<(State, Mino)> {
    let mino = Mino::new(next);

    let possibilities = (0..next.n_rotations())
        .map(|n| state.all_drops(mino.rotated(n)))
        .flatten();

    possibilities.max_by(|(a, _): &(State, Mino), (b, _): &(State, Mino)| {
        f64_cmp(bot.evaluate(a), bot.evaluate(b))
    })
}

/// Simulate `n` games played by `bot` and return the average score.
pub fn simulate<T: Bot>(n: u32, bot: &T) -> f64 {
    let mut sum = 0.0;

    for _ in 0..n {
		let mut state = State::new();
        // Simulated kill-screen at 300 lines
        // MARK: Not in line with real NES Tetris
        while state.lines < 300 {
            let next = random_mino();
            state = match turn(&state, next, bot) {
                Some((state, _)) => state,
                None => { break; }
            };
        }

        sum += state.score as f64;
    }

    sum / (n as f64)
}