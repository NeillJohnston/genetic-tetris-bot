mod simulator;
mod genetic;

use tetris::*;
use simulator::*;
use genetic::*;

use rand::random;

/// A very simple bot that takes three heuristics into account:
/// 	1. Score
/// 	2. Max board height
/// 	3. Holiness (amount of empty space below a block)
struct Simple {
	weights: [f64; 3]
}

impl Simple {
	fn new() -> Simple {
		// Random genes are numbers between -100 and 100
		// (only multiplied by 100 to make analyzing easier)
		let gene = || (random::<f64>() - 0.5) * 200.0;

		Simple {
			weights: [
				gene(),
				gene(),
				gene()
			]
		}
	}

	// Helper for `evaluate`
	fn column_holiness(column: [bool; 20]) -> usize {
		let mut under = false;
		let mut ans = 0;

		for y in 0..20 {
			if !under && column[y] {
				under = true;
			}
			else if under && !column[y] {
				ans += 1;
			}
		}

		ans
	}

	// Helper for `crossover`
	fn from_mask(p1: &Simple, p2: &Simple, mask: u64) -> Simple {
		let weight = |i| if mask & (1u64<<i) > 0 { p1.weights[i] } else { p2.weights[i] };
		Simple {
			weights: [
				weight(0),
				weight(1),
				weight(2)
			]
		}
	}
}

impl Bot for Simple {
	fn evaluate(&self, state: State) -> f64 {
		let score = state.score as f64;

		let max_height = (0..10)
			.map(|x| state.column_depth(x))
			.max().unwrap() as f64;

		let holiness = (0..10)
			.map(|x| Simple::column_holiness(state.column(x)))
			.fold(0, |a, h| a + h) as f64;

		let values = [score, max_height, holiness];
		
		values.iter().zip(self.weights.iter())
			.fold(0.0, |a, (v, w)| a + (v*w))
	}
}

impl Individual for Simple {
	fn fitness(&self) -> f64 {
		// Simulate 10 games to get a somewhat-accurate idea of how well this
		// bot performs
		simulate(10, self)
	}

	// Assign genes (weights) according to opposite non-zero bitmasks
	fn crossover(p1: &Simple, p2: &Simple) -> (Simple, Simple) {
		let mut mask = 0;
		while mask != 0b000 && mask != 0b111 {
			mask = random::<u64>() % 8;
		}

		(Simple::from_mask(p1, p2, mask), Simple::from_mask(p2, p1, mask))
	}

	// Randomly mutate one gene by up to 10%.
	fn mutate(self) -> Simple {
		let mut mutated = self;

		let i = random::<usize>() % 3;
		let p = (random::<f64>() - 0.5) * 20.0;
		mutated.weights[i] += mutated.weights[i] * p;
		
		mutated
	}
}

fn main() {
	// Generate a random initial population
	let n = 100;
	let population: Vec<Simple> = (0..n)
		.map(|_| Simple::new())
		.collect();
	
	// Evolve a bit
	let k = 10;
	let population = evolve(population, basic_generation_iter, k);

	// Find the best individual
	let (champion, fitness) = best(population);
	
	println!("{:?} -> {}", champion.weights, fitness);
}
