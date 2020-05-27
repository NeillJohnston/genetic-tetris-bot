#[path = "util.rs"]
mod util;

use util::f64_cmp;

/// Genetically evolvable individuals.
pub trait Individual {
	/// Evaluate the fitness of this individual - give it a score, where
	/// higher = more likely to survive and create offspring.
	fn fitness(&self) -> f64;

	/// Cross two parents to create two children. Order of the parents should
	/// not be important.
	fn crossover(p1: &Self, p2: &Self) -> (Self, Self)
		where Self: Sized;

	/// Slightly mutate the genes of this individual.
	fn mutate(self) -> Self;
}

/// A simple default evolution step.
/// 
/// After taking the top individuals from the population, forms each possible
/// pair of surviving individuals to make children and then mutates the
/// parents and adds them back to the population. If the original population
/// size is `n`, then the number of survivors chosen is sqrt(`n`).
pub fn basic_generation_iter<T: Individual>(population: Vec<T>) -> Vec<T> {
	let m = (population.len() as f64).sqrt().round() as usize;

	// Sort the population by fitness and retain the top `m`
	let mut fitnesses: Vec<(f64, T)> = population.into_iter()
		.map(|x| (x.fitness(), x))
		.collect();
	fitnesses.sort_by(|(f1, _), (f2, _)| f64_cmp(*f1, *f2).reverse());
	fitnesses.truncate(m);

	let survivors: Vec<T> = fitnesses.into_iter()
		.map(|(_, individual)| individual)
		.collect();

	let mut crossed_over = Vec::new();
	for (i, p1) in survivors.iter().enumerate() {
		for p2 in &survivors[..i] {
			let (c1, c2) = T::crossover(p1, p2);
			crossed_over.push(c1);
			crossed_over.push(c2);
		}
	}

	let mut mutated: Vec<T> = survivors.into_iter()
		.map(|x| x.mutate())
		.collect();

	let mut population = Vec::new();
	population.append(&mut crossed_over);
	population.append(&mut mutated);

	population
}

pub fn evolve<T: Individual>(population: Vec<T>, generation_iter: fn(Vec<T>) -> Vec<T>, n_generations: u32) -> Vec<T> {
	let mut population = population;

	for _ in 0..n_generations {
		population = generation_iter(population);
	}

	population
}

pub fn best<T: Individual>(population: Vec<T>) -> (T, f64) {
	population.into_iter()
		.map(|x| { let f = x.fitness(); (x, f) })
		.max_by(|(_, f1), (_, f2)| f64_cmp(*f1, *f2))
		.unwrap()
}