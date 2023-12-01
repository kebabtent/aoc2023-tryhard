use common::{read_lines, TupleSum};

fn main() {
	let w = [
		"0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
		"five", "six", "seven", "eight", "nine",
	];

	let (a, b) = read_lines()
		.map(|l| {
			let i = w.iter().enumerate().map(|(i, &w)| {
				let mut t = l.match_indices(w).map(|m| m.0);
				let f = t.next();
				(i as u32, f.unwrap_or(!0), t.last().or(f))
			});

			let j = i.clone().filter(|&(i, _, _)| i < 10);
			let f = j.clone().min_by_key(|&(_, f, _)| f).unwrap().0;
			let a = f * 10 + j.max_by_key(|&(_, _, l)| l).unwrap().0;
			let f = i.clone().min_by_key(|&(_, f, _)| f).unwrap().0 % 10;
			let b = f * 10 + i.max_by_key(|&(_, _, l)| l).unwrap().0 % 10;
			(a, b)
		})
		.tuple_sum();
	println!("{a}");
	println!("{b}");
}
