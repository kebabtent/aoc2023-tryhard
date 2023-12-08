use common::{read_all_lines, IterExt, Triplet, TupleMin};
use std::cmp::{max, min};

fn g(m: &Vec<Vec<Triplet<u64>>>, i: u64) -> u64 {
	m.iter().fold(i, |x, v| {
		v.iter()
			.find(|&&(_, s, l)| x >= s && x < s + l)
			.map(|&(d, s, _)| d + x - s)
			.unwrap_or(x)
	})
}

fn main() {
	let mut l = read_all_lines();
	let s = l.next().unwrap();

	let m: Vec<_> = l
		.skip(1)
		.batching(|l| {
			let mut v = l
				.skip(1)
				.take_while(|l| !l.is_empty())
				.filter_map(|l| l.split(" ").filter_map(|v| v.parse().ok()).next_tuple())
				.collect::<Vec<Triplet<u64>>>();
			v.sort_by_key(|&(_, s, _)| s);
			let (_, v) = v.into_iter().fold((0, vec![]), |(p, mut v), (d, s, l)| {
				if s > p {
					v.push((p, p, s - p));
				}
				v.push((d, s, l));
				(s + l, v)
			});
			Some(v)
		})
		.collect();

	let (a, b) = s
		.split(" ")
		.skip(1)
		.filter_map(|v| v.parse::<u64>().ok())
		.tuple_iter()
		.map(|(x, y)| {
			let a = min(g(&m, x), g(&m, y));

			let b = m
				.iter()
				.fold(vec![(x, y)], |r, m| {
					m.iter()
						.map(|&(d, s, l)| {
							r.iter()
								.filter(move |&&(x, y)| s < x + y && s + l > x)
								.map(move |&(x, y)| (max(s, x), min(s + l, x + y)))
								.filter(move |&(p, q)| q > p)
								.map(move |(p, q)| (p - s + d, q - p))
						})
						.flatten()
						.collect()
				})
				.into_iter()
				.map(|(x, _)| x)
				.min()
				.unwrap_or(u64::MAX);

			(a, b)
		})
		.tuple_min()
		.unwrap();

	println!("{a}");
	println!("{b}");
}
