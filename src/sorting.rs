//! Verschiedenste Sortier-Algorithmen.
use std::fmt::Display;

/// Es erscheint passend, mit diesem Algorithmus anzufangen. Einfacher Bubble-Sort,
/// in all seinem Glanz.
pub fn bubble_sort<T: PartialOrd>(data: &mut [T]) {
	if data.len() <= 1 { return; }	// Da gibt's nichts zu machen

	// Alles was einen höheren Index als r_sorted hat ("rechts ist") ist bereits sortiert.
	let mut r_sorted = data.len() - 1;

	while r_sorted > 0 {
		for i in 0...r_sorted - 1 {
			if data[i] > data[i+1] {
				data.swap(i, i+1);
			}
		}

		r_sorted -= 1;
	}
}

/// Insertion sort Implementation.
pub fn insertion_sort<T: PartialOrd>(data: &mut [T]) {
	if data.len() <= 1 { return; }

	let mut l_sorted = 1;
	while l_sorted < data.len() {
		let mut test_i = l_sorted;
		while test_i > 0 && data[test_i-1] > data[test_i] {
			data.swap(test_i-1, test_i);
			test_i -= 1
		}

		l_sorted += 1;
	}
}

pub fn quick_sort<T: PartialOrd + Copy>(data: &mut [T]) {
	let data_len = data.len();
	if data_len <= 1 { return; }

	quick_sort_sub(data, 0, data_len as i64 - 1);
}

fn quick_sort_sub<T: PartialOrd + Copy>(data: &mut [T], l: i64, r: i64) {
	if l < r {
		let p = partition(data, l, r);
		quick_sort_sub(data, l, p);
		quick_sort_sub(data, p+1, r);
	}
}

fn partition<T: PartialOrd + Copy>(data: &mut [T], l: i64, r: i64) -> i64 {
	let comp = data[l as usize];

	let mut i = l-1;
	let mut j = r+1;

	loop {
		i += 1;
		j -= 1;

		while data[i as usize] < comp { i += 1; }
		while data[j as usize] > comp { j -= 1; }

		if i >= j { return j; }

		data.swap(i as usize, j as usize);
	}
}
