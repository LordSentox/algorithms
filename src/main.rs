#![feature(inclusive_range_syntax)]

extern crate rand;
extern crate time;
use rand::distributions::{IndependentSample, Range};
use time::PreciseTime;

pub mod biggest_difference;
pub mod sorting;

pub const DATA_SIZE: usize = /*200*1024*/ 50;

pub fn random_data_set(min: i32, max: i32, size: usize) -> Vec<i32> {
	let mut data = Vec::with_capacity(size);

	let between = Range::new(min, max);
   	let mut rng = rand::thread_rng();

	for _ in 0..size-1 {
		data.push(between.ind_sample(&mut rng));
	}

	data
}

fn print_data_inf(data: &[i32]) {
	println!("Testmenge: {}kB", data.len()*4/1024);

	if data.len() <= 50 {
		println!("Data: {:?}", &data);
	}
}

fn test_biggest_difference(data: Vec<i32>) -> Vec<i32> {
	print_data_inf(&data);

	let start_time = PreciseTime::now();
	let solution = biggest_difference::run(&data);
	let end_time = PreciseTime::now();

	println!("Größte Differenz in positive Index-Richtung: {} [Laufzeit: {}]", solution, start_time.to(end_time));
	data
}

fn test_bubble_sort(mut data: Vec<i32>) {
	println!("Teste Bubble-Sort");
	print_data_inf(&data);

	let start_time = PreciseTime::now();
	sorting::bubble_sort(&mut data);
	let end_time = PreciseTime::now();

	println!("Daten sortiert durch Bubble-Sort. [Laufzeit: {}]", start_time.to(end_time));
	print_data_inf(&data);
}

fn test_insertion_sort(mut data: Vec<i32>) {
	println!("Teste Insertion-Sort");

	let start_time = PreciseTime::now();
	sorting::insertion_sort(&mut data);
	let end_time = PreciseTime::now();

	println!("Daten sortiert durch Insertion-Sort. [Laufzeit: {}]", start_time.to(end_time));
	print_data_inf(&data);
}

fn main() {
	// Test des "biggest_difference"-Algorithmus.
	let data = random_data_set(0, 1000, DATA_SIZE);
	let data = test_biggest_difference(data);
	println!("");

	test_bubble_sort(data);
	println!("");

	test_insertion_sort(random_data_set(0, 1000, DATA_SIZE));
	println!("");
}
