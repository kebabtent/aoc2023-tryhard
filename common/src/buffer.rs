use std::collections::VecDeque;
use std::ops::Index;

pub struct Buffer<T> {
	capacity: usize,
	inner: VecDeque<T>,
}

impl<T> Buffer<T> {
	pub fn new(capacity: usize) -> Self {
		Self {
			capacity,
			inner: VecDeque::with_capacity(capacity),
		}
	}

	pub fn capacity(&self) -> usize {
		self.capacity
	}

	pub fn len(&self) -> usize {
		self.inner.len()
	}

	pub fn is_full(&self) -> bool {
		self.capacity == self.inner.len()
	}

	pub fn iter(&self) -> impl Iterator<Item = &T> {
		self.inner.iter()
	}

	pub fn get(&self, index: usize) -> Option<&T> {
		self.inner.get(index)
	}

	pub fn binary_search(&self, x: &T) -> Result<usize, usize>
	where
		T: Ord,
	{
		self.inner.binary_search(x)
	}

	pub fn push(&mut self, value: T) -> Option<T> {
		let res = if self.is_full() {
			self.inner.pop_front()
		} else {
			None
		};
		self.inner.push_back(value);
		res
	}

	pub fn insert_sorted(&mut self, value: T)
	where
		T: Ord,
	{
		let index = self.binary_search(&value).unwrap_or_else(|e| e);
		if index == 0 && self.is_full() {
			return;
		}
		if self.is_full() {
			self.inner.pop_front();
		}
		self.inner.insert(index.saturating_sub(1), value);
	}
}

impl<T> Index<usize> for Buffer<T> {
	type Output = T;

	fn index(&self, index: usize) -> &Self::Output {
		self.get(index).unwrap()
	}
}
