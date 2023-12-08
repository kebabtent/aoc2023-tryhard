use std::marker::PhantomData;
use std::mem::swap;
use std::ops::Add;

pub type Doublet<T> = (T, T);
pub type Triplet<T> = (T, T, T);
pub type Quartet<T> = (T, T, T, T);
pub type Array<const N: usize, T> = [T; N];

pub trait Tuple<T>: Clone + Sized {
	fn collect<I: Iterator<Item = T>>(it: &mut I) -> Option<Self>;
	fn push_right(&mut self, v: T);
}

impl<T: Clone> Tuple<T> for Doublet<T> {
	fn collect<I: Iterator<Item = T>>(it: &mut I) -> Option<Self> {
		Some((it.next()?, it.next()?))
	}

	fn push_right(&mut self, v: T) {
		swap(&mut self.0, &mut self.1);
		self.1 = v;
	}
}

impl<T: Clone> Tuple<T> for Triplet<T> {
	fn collect<I: Iterator<Item = T>>(it: &mut I) -> Option<Self> {
		Some((it.next()?, it.next()?, it.next()?))
	}

	fn push_right(&mut self, v: T) {
		swap(&mut self.0, &mut self.1);
		swap(&mut self.1, &mut self.2);
		self.2 = v;
	}
}

impl<T: Clone> Tuple<T> for Quartet<T> {
	fn collect<I: Iterator<Item = T>>(it: &mut I) -> Option<Self> {
		Some((it.next()?, it.next()?, it.next()?, it.next()?))
	}

	fn push_right(&mut self, v: T) {
		swap(&mut self.0, &mut self.1);
		swap(&mut self.1, &mut self.2);
		swap(&mut self.2, &mut self.3);
		self.3 = v;
	}
}

impl<const N: usize, T: Copy + Default> Tuple<T> for [T; N] {
	fn collect<I: Iterator<Item = T>>(it: &mut I) -> Option<Self> {
		let mut arr = [T::default(); N];
		for i in 0..N {
			arr[i] = it.next()?;
		}
		Some(arr)
	}

	fn push_right(&mut self, v: T) {
		for i in 0..(N - 1) {
			self.swap(i, i + 1);
		}
		self[N - 1] = v;
	}
}

pub struct TupleIter<I, T> {
	pub(crate) iter: I,
	pub(crate) _tuple: PhantomData<T>,
}

impl<I, T, U> Iterator for TupleIter<I, T>
where
	I: Iterator<Item = U>,
	T: Tuple<U>,
{
	type Item = T;

	fn next(&mut self) -> Option<T> {
		T::collect(&mut self.iter)
	}
}

pub struct TupleWindows<I, T> {
	pub(crate) iter: I,
	pub(crate) tuple: Option<T>,
}

impl<I, T, U> Iterator for TupleWindows<I, T>
where
	I: Iterator<Item = U>,
	T: Tuple<U>,
	U: Copy,
{
	type Item = T;

	fn next(&mut self) -> Option<T> {
		let last = self.tuple.as_mut()?;
		last.push_right(self.iter.next()?);
		Some(last.clone())
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(0, None)
	}
}

pub trait TupleSum<T, U, V>
where
	T: Tuple<U>,
{
	type Output: Tuple<V>;
	fn tuple_sum(self) -> Self::Output;
}

impl<T, U> TupleSum<Doublet<U>, U, U> for T
where
	T: Iterator<Item = Doublet<U>>,
	U: Add<U, Output = U> + Default + Clone,
{
	type Output = Doublet<U>;
	fn tuple_sum(self) -> Self::Output {
		self.fold((U::default(), U::default()), |a, x| (a.0 + x.0, a.1 + x.1))
	}
}

impl<T> TupleSum<Doublet<bool>, bool, u32> for T
where
	T: Iterator<Item = Doublet<bool>>,
{
	type Output = Doublet<u32>;
	fn tuple_sum(self) -> Self::Output {
		self.fold((0, 0), |a, x| (a.0 + x.0 as u32, a.1 + x.1 as u32))
	}
}

pub trait TupleMin<T, U>
where
	T: Tuple<U>,
{
	type Output: Tuple<U>;
	fn tuple_min(self) -> Option<Self::Output>;
}

impl<T, U> TupleMin<Doublet<U>, U> for T
where
	T: Iterator<Item = Doublet<U>>,
	U: Ord + Clone,
{
	type Output = Doublet<U>;
	fn tuple_min(self) -> Option<Self::Output> {
		self.reduce(|(a, b), (x, y)| (a.min(x), b.min(y)))
	}
}
