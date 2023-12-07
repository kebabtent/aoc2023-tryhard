use common::read_lines;
use std::collections::VecDeque;

fn main() {
	let (a, b, _) = read_lines()
		.filter_map(|l| {
			l.split(&[':', '|'])
				.skip(1)
				.map(|t| {
					t.split(" ")
						.filter_map(|n| n.parse::<u32>().ok())
						.map(|n| 1 << n)
						.sum::<u128>()
				})
				.reduce(|x, y| x & y)
				.map(|v| v.count_ones())
		})
		.fold(
			(0u32, 0, VecDeque::<u32>::new()),
			|(mut a, mut b, mut c), o| {
				if o > 0 {
					a += 1 << (o - 1);
				}
				let n = c.pop_front().unwrap_or(0) + 1;
				b += n;
				let mut r = 0..o;
				c.iter_mut().zip(&mut r).for_each(|(v, _)| *v += n);
				r.for_each(|_| c.push_back(n));
				(a, b, c)
			},
		);
	println!("{a}");
	println!("{b}");
}
