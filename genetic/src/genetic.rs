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

/// Statistical summary of a population.
#[derive(Debug)]
pub struct Summary {
	// 5-number summary of fitnesses
	fitness_distribution: [f64; 5]
}

/// Transform a population (vector of `Individuals`) into a sorted list that
/// pairs individuals with their fitness.
/// 
/// Runs multithreaded in order to speed up computation, because evaluating an
/// individual's fitness is a costly operation.
fn population_fitness<T: Individual>(population: Vec<T>) -> Vec<(f64, T)> {
	let handles: Vec<JoinHandle<(f64, T)>> = population.into_iter()
		.map(|individual| spawn(|| (individual.fitness(), individual)))
		.collect();
	
	let mut fitnesses: Vec<(f64, T)> = handles.into_iter()
		.map(|handle| handle.join().unwrap())
		.collect();
		
	fitnesses.sort_by(|(f1, _), (f2, _)| f64_cmp(*f1, *f2).reverse());

	fitnesses
}

/// A simple default evolution step.
/// 
/// After taking the top individuals from the population, forms each possible
/// pair of surviving individuals to make children and then mutates the
/// parents and adds them back to the population. If the original population
/// size is `n`, then the number of survivors chosen is sqrt(`n`).
/// 
/// While this method has some obvious drawbacks (max fitness can decrease due
/// to mutations, and often will) it's a starting point.
/// 
/// Returns the next population as well as a summary of the initial population.
pub fn basic_generation_iter<T: Individual>(population: Vec<T>) -> (Vec<T>, Summary) {
	let n = population.len();
	let m = (n as f64).sqrt().round() as usize;

	// Sort the population by fitness and retain the top `m`
	let mut fitnesses = population_fitness(population);

	let summary = Summary {
		fitness_distribution: [
			fitnesses[n-1].0,
			fitnesses[3*n/4].0,
			fitnesses[n/2].0,
			fitnesses[n/4].0,
			fitnesses[0].0
		]
	};

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

	(population, summary)
}