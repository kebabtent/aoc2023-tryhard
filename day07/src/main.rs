use common::{read_lines, IterExt, TupleSum};
use std::cmp::max;

fn main() {
	let r = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
	let mut h: Vec<_> = read_lines()
		.filter_map(|l| {
			l.split(" ")
				.next_tuple()
				.map(|(c, w)| {
					let (c, n) = c
						.chars()
						.rev()
						.filter_map(|c| r.iter().rev().position(|&r| r == c))
						.enumerate()
						.map(|(i, r)| (r << 4 * i, 1 << 4 * r))
						.tuple_sum();
					let (m, d) = (0..13)
						.map(|i| (n >> 4 * i) & 0xF)
						.fold((0, 0), |(m, d), x| (max(m, x), d + (x == 2) as usize));
					w.parse::<usize>().ok().map(|w| (c + (3 * m + d << 20), w))
				})
				.flatten()
		})
		.collect();
	h.sort_by_key(|&(c, _)| c);

	let a: usize = h.iter().enumerate().map(|(i, &(_, w))| (i + 1) * w).sum();

	println!("{a}");
}
