use std::cmp::Ord;

use crate::Order;

pub fn selection_sort<T: Clone + Copy + Ord>(input: &[T], order: Order) -> Vec<T> {
	let compare: fn(T, T) -> bool = match order {
		Order::Ascending => |a, b| a > b,
		Order::Descending => |a, b| a < b,
	};
	let mut list = input.to_vec();
	let len = input.len();
	for i in 0..len {
		let mut min_idx = i;
		for j in (i + 1)..len {
			if compare(list[min_idx], list[j]) {
				min_idx = j;
			}
		}

		list.swap(i, min_idx);
	}

	list
}

#[cfg(test)]
mod tests {
	use super::*;

	use crate::testing::{UNSORTED_LIST, check_asc, check_desc};

	#[test]
	fn can_sort() {
		let list = Vec::from(UNSORTED_LIST);
		let sorted = selection_sort(&list, Order::Ascending);

		assert_eq!(sorted.len(), list.len());
		check_asc(sorted);
	}

	#[test]
	fn can_sort_descending() {
		let list = Vec::from(UNSORTED_LIST);
		let sorted = selection_sort(&list, Order::Descending);

		assert_eq!(sorted.len(), list.len());
		check_desc(sorted);
	}
}
