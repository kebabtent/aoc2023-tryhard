use common::{read_lines, TupleSum};

fn main() {
	let w = [
		"zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1",
		"2", "3", "4", "5", "6", "7", "8", "9",
	];

	let (a, b) = read_lines()
		.map(|l| {
			let mut t = l.chars().filter(|c| c.is_numeric()).map(|c| c as u32 - 48);
			let a = t.next().unwrap();

			let b = w
				.iter()
				.enumerate()
				.map(|(i, &w)| {
					let mut t = l.match_indices(w);
					let f = t.next().map(|f| f.0);
					(i as u32 % 10, f, t.last().map(|l| l.0).or(f))
				})
				.fold((Some(usize::MAX), 0, None, 0), |mut c, (i, f, l)| {
					if f < c.0 && f.is_some() {
						c.0 = f;
						c.1 = i;
					}
					if l > c.2 {
						c.2 = l;
						c.3 = i;
					}
					c
				});

			(a * 10 + t.last().unwrap_or(a), b.1 * 10 + b.3)
		})
		.tuple_sum();
	println!("{a}");
	println!("{b}");
}
