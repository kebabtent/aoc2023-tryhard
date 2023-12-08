use common::{read_lines, IterExt};

fn f(l: &str) -> impl Iterator<Item = u64> + '_ {
	l.split(" ")
		.filter(|&p| !p.is_empty())
		.filter_map(|p| p.parse().ok())
}

fn g(t: u64, d: u64) -> Option<u64> {
	let mut i = (0..t).filter(|&x| x * (t - x) > d).rev();
	Some(1 + i.next()? - i.rev().next()?)
}

fn h(l: &str) -> u64 {
	l.chars()
		.filter(|c| c.is_digit(10))
		.fold(0, |a, c| a * 10 + c.to_digit(10).unwrap() as u64)
}

fn main() {
	let mut l = read_lines();
	let (t, d) = l.next_tuple().unwrap();

	let a: u64 = f(&t).zip(f(&d)).filter_map(|(t, d)| g(t, d)).product();
	let b = g(h(&t), h(&d)).unwrap();

	println!("{a}");
	println!("{b}");
}
