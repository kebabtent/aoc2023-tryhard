use common::{read_lines, TupleSum};

fn main() {
	let w = [
		"zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1",
		"2", "3", "4", "5", "6", "7", "8", "9",
	];

	let (a, b) = read_lines()
		.map(|l| {
			let mut t = l.chars().filter(|c| c.is_numeric()).map(|c| c as u32 - 48);
			let f = t.next().unwrap();
			let a = f * 10 + t.last().unwrap_or(f);

			let i = w.iter().enumerate().map(|(i, &w)| {
				let mut t = l.match_indices(w).map(|m| m.0);
				let f = t.next();
				(i as u32 % 10, f.unwrap_or(!0), t.last().or(f))
			});

			let f = i.clone().min_by_key(|&(_, f, _)| f).unwrap().0;
			let b = f * 10 + i.max_by_key(|&(_, _, l)| l).unwrap().0;
			(a, b)
		})
		.tuple_sum();
	println!("{a}");
	println!("{b}");
}
