extern crate rand;
extern crate time;
use rand::distributions::{IndependentSample, Range};
use time::PreciseTime;

pub mod biggest_difference;

pub const DATA_SIZE: usize = 100*1024*1024;

pub fn random_data_set(min: i32, max: i32, size: usize) -> Vec<i32> {
	let mut data = Vec::with_capacity(size);

	let between = Range::new(min, max);
   	let mut rng = rand::thread_rng();

	for _ in 0..size-1 {
		data.push(between.ind_sample(&mut rng));
	}

	data
}

fn main() {
	// Test des "biggest_difference"-Algorithmus.
	let data = random_data_set(0, 20, /*DATA_SIZE*/10);
	println!("Test data size: {}kB", DATA_SIZE*4/1024);

	if data.len() <= 50 {
		println!("Data: {:?}", &data);
	}

	let start_time = PreciseTime::now();
	let solution = biggest_difference::run(&data);
	let end_time = PreciseTime::now();

	println!("Größte Differenz in positive Index-Richtung: {} [Laufzeit: {}s]", solution, start_time.to(end_time));
}
