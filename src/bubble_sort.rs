use std::cmp::Ord;

use crate::Order;

pub fn bubble_sort<T: Copy + Ord>(input: &[T], order: Order) -> Vec<T> {
	let should_swap: fn(T, T) -> bool = match order {
		Order::Ascending => |a, b| a > b,
		Order::Descending => |a, b| a < b,
	};
	let mut sorted = false;
	let mut list = input.to_vec();
	let len = input.len();
	while !sorted {
		sorted = true;
		for i in 0..=(len - 2) {
			if should_swap(list[i], list[i + 1]) {
				list.swap(i, i + 1);
				sorted = false;
			}
		}
	}

	list
}

pub trait BubbleSortable<T: Copy + Ord> {
	fn bubble_sort(&self, order: Order) -> Self;
}

impl<T: Copy + Ord> BubbleSortable<T> for Vec<T> {
	fn bubble_sort(&self, order: Order) -> Vec<T> {
		bubble_sort(self, order)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	use crate::testing::{UNSORTED_LIST, check_asc, check_desc};

	#[test]
	fn can_sort() {
		let list = Vec::from(UNSORTED_LIST);
		let sorted = bubble_sort(&list, Order::Ascending);

		assert_eq!(sorted.len(), list.len());
		check_asc(sorted);
	}

	#[test]
	fn can_sort_descending() {
		let list = Vec::from(UNSORTED_LIST);
		let sorted = bubble_sort(&list, Order::Descending);

		assert_eq!(sorted.len(), list.len());
		check_desc(sorted);
	}

	#[test]
	fn can_sort_via_trait() {
		let list = Vec::from(UNSORTED_LIST);
		let sorted = list.bubble_sort(Order::Ascending);

		assert_eq!(sorted.len(), list.len());
		check_asc(sorted);
	}
}
