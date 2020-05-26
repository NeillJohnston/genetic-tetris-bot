use tetris::*;
use rand::random;

trait Decider {
    fn decide(&self, state: State) -> f64;
}

fn decide<Bot: Decider>(state: State, next: MinoShape, bot: &Bot) -> Option<State> {
    let mino = Mino::new(next);

    let possibilities = (0..next.n_rotations())
        .map(|n| state.all_drops(mino.rotated(n)))
        .flatten();

    // Have to do this because f64 does not have cmp 
    possibilities.max_by(|a: &State, b: &State| {
        if bot.decide(*a) < bot.decide(*b) {
            std::cmp::Ordering::Less
        }
        else {
            std::cmp::Ordering::Greater
        }
    })
}

/// Simulate `n` games played by `bot` and return the average score.
fn simulate<Bot: Decider>(n: u32, bot: &Bot) -> f64 {
    let mut aggregate = 0.0;

    for _ in 0..n {
        let mut state = State::new();
        loop {
            let next = random_mino();
            state = match decide(state, next, bot) {
                Some(state) => state,
                None => { break; }
            };
        }

        aggregate += state.score as f64;
    }

    aggregate / (n as f64)
}

#[derive(Clone, Copy, Debug)]
struct Basic {
    weights: [f64; 3]
}

impl Basic {
    fn new(weights: [f64; 3]) -> Basic {
        Basic {
            weights
        }
    }
}

impl Decider for Basic {
    fn decide(&self, state: State) -> f64 {
        let height = (0..10)
            .map(|x| {
                let d = 20 - state.column_depth(x);
                d*d
            })
            .fold(0, |a, x| a + x);

        // Score calculation - a weighted sum
        let heuristics = vec![
            (state.score as f64) / 10000.0,
            state.lines as f64,
            (height as f64) * 10.0
        ];

        heuristics.iter().zip(self.weights.iter())
            .fold(0.0, |a, (h, w)| a + h*w)
    }
}

fn random_mino() -> MinoShape {
    match (random::<u32>()) % 7 {
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

fn main() {
    let basic = Basic::new([random::<f64>() - 0.5, random::<f64>() - 0.5, random::<f64>() - 0.5]);
    let score = simulate(5, &basic);

    println!("{:?} -> {}", basic.weights, score);
}
