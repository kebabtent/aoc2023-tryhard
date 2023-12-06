use common::{read_lines, Doublet, IterExt, TupleSum};
use std::cmp::max;

fn main() {
	let m = [12, 13, 14];
	let (a, b) = read_lines()
		.enumerate()
		.map(|(i, l)| {
			let x = l
				.split(": ")
				.skip(1)
				.next()
				.unwrap()
				.split("; ")
				.map(|g| {
					g.split(" ")
						.tuple_iter::<Doublet<_>>()
						.filter_map(|(n, c)| {
							Some((n.parse::<u32>().ok()?, c.chars().next()? as usize % 3))
						})
						.fold([0; 3], |mut s, (n, c)| {
							s[c] = n;
							s
						})
				})
				.fold([0; 3], |mut a, b| {
					a.iter_mut().zip(b).for_each(|(a, b)| *a = max(*a, b));
					a
				});
			(
				x.iter()
					.zip(m)
					.all(|(&a, b)| a <= b)
					.then_some(i + 1)
					.unwrap_or(0),
				x.into_iter().map(|v| v as usize).product(),
			)
		})
		.tuple_sum();
	println!("{a}");
	println!("{b}");
}
