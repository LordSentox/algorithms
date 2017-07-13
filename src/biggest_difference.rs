///! Teile und Herrsche Implementation eines Algorithmus, der die größte Differenz zweier Zahlen
///! sucht, welche hintereinander liegen. z.B. ein Datenset von:
///! [1, 8, 4, 2] würde 6 ausgeben, nämlich die Differenz zwischen 8 und 2. (1 und 8 wäre -7, welches
///! als kleiner gewertet werden soll.)

/// Rekursive Implementation des Algorithmus in einem einzelnen thread.
pub fn run(data: &[i32]) -> u16 {
	if data.len() == 0 {
		0
	}
	else {
		run_sub(data, 0, data.len()-1).2
	}
}

#[inline]
fn max<T: PartialOrd>(x: T, y: T) -> T {
	if x > y { x }
	else { y }
}

#[inline]
fn max_3<T: PartialOrd + Copy>(x: T, y: T, z: T) -> T {
	let max_2 = max(x, y);
	max(max_2, z)
}

#[inline]
fn min<T: PartialOrd>(x: T, y: T) -> T {
	if x < y { x }
	else { y }
}

/// Auf einer Unterfolge laufen lassen. Gibt die minimale Zahl, die maximale Zahl und die maximale
/// Differenz (Welche immer positiv ist durch den Fall data[i] - data[i]) zurück.
fn run_sub(data: &[i32], l: usize, r: usize) -> (i32, i32, u16) {
	// Rekursionsabbruch
	if l == r { (data[l], data[l], 0) }
	else {
		let m = (l + r) / 2;
		let (l_max, l_min, l_diff) = run_sub(&data, l, m);
		let (r_max, r_min, r_diff) = run_sub(&data, m+1, r);
		let max = max(l_max, r_max);
		let min = min(l_min, r_min);
		let diff = max_3(l_diff as i32, r_diff as i32, l_max - r_min);

		(max, min, diff as u16)
	}
}
