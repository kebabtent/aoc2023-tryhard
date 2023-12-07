use common::{read_chars, IterExt, Neighbours};
use std::mem::swap;

fn main() {
	let g: Vec<_> = read_chars().collect();
	let m = g.iter().enumerate().find(|&(_, &c)| c == '\n').unwrap().0 + 1;
	let n = g.len() / m;

	let e: Vec<_> = g
		.iter()
		.enumerate()
		.filter(|&(_, &c)| c == '*')
		.map(|(i, _)| i)
		.collect();
	let mut r = vec![vec![]; e.len()];

	let a: usize = g
		.iter()
		.enumerate()
		.batching(|t| {
			t.fold_while((0, false, vec![]), |(s, f, h), (i, &c)| {
				if let Some(d) = c.to_digit(10).map(|d| d as usize) {
					*s = *s * 10 + d;
					let (p, q) = (i % m, i / m)
						.neighbours(m - 1, n)
						.map(|(x, y)| x + y * m)
						.map(|n| (n, g[n]))
						.fold((false, vec![]), |(f, mut r), (n, c)| {
							if c == '*' {
								r.push(n);
							}
							(f || (!c.is_digit(10) && !['.', '\n'].contains(&c)), r)
						});
					*f |= p;
					h.extend(q);
					None
				} else {
					if *s > 0 {
						h.sort();
						h.dedup();
						h.drain(..)
							.filter_map(|n| e.binary_search(&n).ok())
							.for_each(|n| {
								r[n].push(*s);
							});
					}

					Some((*s, *f)).filter(|&(s, _)| s > 0)
				}
			})
		})
		.filter_map(|(s, f)| f.then_some(s))
		.sum();

	let b: usize = r
		.into_iter()
		.filter(|r| r.len() == 2)
		.map(|r| r.into_iter().product::<usize>())
		.sum();

	println!("{a}");
	println!("{b}");
}
