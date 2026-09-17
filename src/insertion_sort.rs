use std::cmp::Ord;

use crate::Order;

pub fn insertion_sort<T: Copy + Ord>(input: &[T], order: Order) -> Vec<T> {
	let compare: fn(T, T) -> bool = match order {
		Order::Ascending => |a, b| a > b,
		Order::Descending => |a, b| a < b,
	};
	let mut list = input.to_vec();
	let len = input.len();

	for i in 1..len {
		let key = list[i];
		let mut j = i as i32 - 1;

		while j >= 0 && compare(list[j as usize], key) {
			list[j as usize + 1] = list[j as usize];
			j -= 1;
		}

		list[(j + 1) as usize] = key;
	}

	list
}

pub trait InsertionSortable<T: Copy + Ord> {
	fn insertion_sort(&self, order: Order) -> Self;
}

impl<T: Copy + Ord> InsertionSortable<T> for Vec<T> {
	fn insertion_sort(&self, order: Order) -> Vec<T> {
		insertion_sort(self, order)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::testing::{UNSORTED_LIST, check_asc, check_desc};

	#[test]
	fn can_sort() {
		let list = Vec::from(UNSORTED_LIST);
		let sorted = insertion_sort(&list, Order::Ascending);

		assert_eq!(sorted.len(), list.len());
		check_asc(sorted);
	}

	#[test]
	fn can_sort_descending() {
		let list = Vec::from(UNSORTED_LIST);
		let sorted = insertion_sort(&list, Order::Descending);

		assert_eq!(sorted.len(), list.len());
		check_desc(sorted);
	}

	#[test]
	fn can_sort_via_trait() {
		let list = Vec::from(UNSORTED_LIST);
		let sorted = list.insertion_sort(Order::Ascending);

		assert_eq!(sorted.len(), list.len());
		check_asc(sorted);
	}
}
