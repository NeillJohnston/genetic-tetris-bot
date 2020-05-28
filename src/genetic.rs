#[path = "util.rs"]
mod util;

use std::thread::{JoinHandle, spawn};
use util::f64_cmp;

/// Genetically evolvable individuals.
/// 
/// `Individual` types are bound by `Send` and `'static` so that they may
/// safely be used in threads.
pub trait Individual: std::marker::Send + 'static {
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

/// Transform a population (vector of `Individuals` into a vector that pairs)
/// individuals with their fitness.
/// 
/// Runs multithreaded in order to speed up computation (because we assume)
/// that evaluating an individual's fitness is a costly operation).
fn all_fitnesses<T: Individual>(population: Vec<T>) -> Vec<(f64, T)> {
	let handles: Vec<JoinHandle<(f64, T)>> = population.into_iter()
		.map(|individual| spawn(|| (individual.fitness(), individual)))
		.collect();
	
	handles.into_iter()
		.map(|handle| handle.join().unwrap())
		.collect()
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
	let mut fitnesses = all_fitnesses(population);
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

/// Evolve a population for `n_generations` generations with a specified
/// generation iterator function.
pub fn evolve<T: Individual>(population: Vec<T>, generation_iter: fn(Vec<T>) -> Vec<T>, n_generations: u32) -> Vec<T> {
	let mut population = population;

	for _ in 0..n_generations {
		population = generation_iter(population);
	}

	population
}

/// Reduce a population to its best individual.
/// TODO: Re-runs fitness evaluation so this definitely needs to be replaced.
pub fn best<T: Individual>(population: Vec<T>) -> (f64, T) {
	all_fitnesses(population).into_iter()
		.max_by(|(f1, _), (f2, _)| f64_cmp(*f1, *f2))
		.unwrap()
}