//! Verschiedenste Sortier-Algorithmen.

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
