pub use self::bitmap::*;
pub use self::buffer::*;
pub use self::tuple::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::once;
use std::marker::PhantomData;
use std::str::FromStr;

mod bitmap;
mod buffer;
mod tuple;

pub fn read_all_lines() -> impl Iterator<Item = String> {
	reader().lines().filter_map(|l| l.ok())
}

pub fn read_lines() -> impl Iterator<Item = String> {
	read_all_lines().filter(|l| !l.is_empty())
}

pub fn try_read_lines_as<T: FromStr>() -> impl Iterator<Item = Result<T, String>> {
	read_lines().map(|l| T::from_str(&l).map_err(|_| l))
}

pub fn read_lines_as<T: FromStr>() -> impl Iterator<Item = T> {
	try_read_lines_as().filter_map(|l| l.ok())
}

pub fn read_chars() -> impl Iterator<Item = char> {
	let it = ReadChars {
		reader: reader(),
		buf: [0],
	};
	it.filter_map(|c| c.ok())
}

fn reader() -> BufReader<File> {
	BufReader::new(
		File::open(format!(
			"input/{}.txt",
			std::env::var("CARGO_PKG_NAME").unwrap()
		))
		.unwrap(),
	)
}

pub trait IterExt: Iterator + Sized {
	fn batching<B, F>(self, f: F) -> Batching<Self, F>
	where
		F: FnMut(&mut Self) -> Option<B>,
	{
		Batching { f, iter: self }
	}

	fn fold_while<S, B, F>(&mut self, mut state: S, mut f: F) -> Option<B>
	where
		Self: Sized,
		F: FnMut(&mut S, Self::Item) -> Option<B>,
	{
		while let Some(item) = self.next() {
			let v = f(&mut state, item);
			if v.is_some() {
				return v;
			}
		}
		None
	}

	fn next_tuple<T: Tuple<Self::Item>>(&mut self) -> Option<T> {
		Tuple::collect(self)
	}

	fn tuple_iter<T>(self) -> TupleIter<Self, T>
	where
		T: Tuple<Self::Item>,
	{
		TupleIter {
			iter: self,
			_tuple: PhantomData,
		}
	}

	fn tuple_windows<T>(mut self) -> TupleWindows<Self, T>
	where
		T: Tuple<Self::Item>,
		Self::Item: Clone,
	{
		let tuple = self
			.next()
			.and_then(|f| T::collect(&mut once(f.clone()).chain(once(f)).chain(&mut self)));

		TupleWindows { iter: self, tuple }
	}
}

impl<T> IterExt for T where T: Iterator {}

pub struct Batching<I, F> {
	f: F,
	iter: I,
}

impl<B, F, I> Iterator for Batching<I, F>
where
	I: Iterator,
	F: FnMut(&mut I) -> Option<B>,
{
	type Item = B;
	fn next(&mut self) -> Option<B> {
		(self.f)(&mut self.iter)
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(0, None)
	}
}

pub struct ReadChars<R> {
	reader: R,
	buf: [u8; 1],
}

impl<R: BufRead> Iterator for ReadChars<R> {
	type Item = std::io::Result<char>;

	fn next(&mut self) -> Option<Self::Item> {
		match self.reader.read(&mut self.buf) {
			Ok(0) => None,
			Ok(_) => Some(Ok(self.buf[0] as char)),
			Err(e) => Some(Err(e)),
		}
	}
}

pub enum Either<L, R> {
	L(L),
	R(R),
}

impl<L, R> Either<L, R> {
	pub fn left(&self) -> &L {
		match self {
			Self::L(l) => l,
			_ => panic!("Not left"),
		}
	}

	pub fn right(&self) -> &R {
		match self {
			Self::R(r) => r,
			_ => panic!("Not right"),
		}
	}

	pub fn left_mut(&mut self) -> &mut L {
		match self {
			Self::L(l) => l,
			_ => panic!("Not left"),
		}
	}

	pub fn right_mut(&mut self) -> &mut R {
		match self {
			Self::R(r) => r,
			_ => panic!("Not right"),
		}
	}
}
