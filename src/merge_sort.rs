use std::cmp::Ord;

use crate::Order;

pub fn merge_sort<T: Copy + Ord>(input: &[T], order: &Order) -> Vec<T> {
	let len = input.len();
	if len == 1 {
		return input.to_vec();
	}

	let mid = ((len - 1) / 2) as usize;

	let a_vec = &input.to_vec()[0..=mid];
	let b_vec = &input.to_vec()[mid + 1..input.len()];

	let a = merge_sort(a_vec, order);
	let b = merge_sort(b_vec, order);

	merge(&a, &b, order)
}

fn merge<T: Copy + Ord>(a: &[T], b: &[T], order: &Order) -> Vec<T> {
	let compare: fn(T, T) -> bool = match order {
		Order::Ascending => |a, b| a <= b,
		Order::Descending => |a, b| a >= b,
	};
	let len_a = a.len();
	let len_b = b.len();
	let list_a = a.to_vec();
	let list_b = b.to_vec();

	let mut i: usize = 0;
	let mut j: usize = 0;

	let mut list = Vec::new();

	// Merge both arrays.
	while i < len_a && j < len_b {
		if compare(list_a[i], list_b[j]) {
			list.push(list_a[i]);
			i += 1;
		} else {
			list.push(list_b[j]);
			j += 1;
		}
	}

	// Copy the remaining elements of list_a, if there are any.
	while i < len_a {
		list.push(list_a[i]);
		i += 1;
	}

	// Copy the remaining elements of list_b, if there are any.
	while j < len_b {
		list.push(list_b[j]);
		j += 1;
	}

	list
}

pub trait MergeSortable<T: Copy + Ord> {
	fn merge_sort(&self, order: Order) -> Self;
}

impl<T: Copy + Ord> MergeSortable<T> for Vec<T> {
	fn merge_sort(&self, order: Order) -> Vec<T> {
		merge_sort(self, &order)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::testing::{UNSORTED_LIST, check_asc, check_desc};

	#[test]
	fn merge_works() {
		let a = Vec::from([2]);
		let b = Vec::from([1]);
		let merged = merge(&a, &b, &Order::Ascending);

		assert_eq!(merged.len(), 2);
		assert_eq!(merged[0], 1);
		assert_eq!(merged[1], 2);
	}

	#[test]
	fn merge_works_descending() {
		let a = Vec::from([1]);
		let b = Vec::from([2]);
		let merged = merge(&a, &b, &Order::Descending);

		assert_eq!(merged.len(), 2);
		assert_eq!(merged[0], 2);
		assert_eq!(merged[1], 1);
	}

	#[test]
	fn can_sort() {
		let list = Vec::from(UNSORTED_LIST);
		let sorted = merge_sort(&list, &Order::Ascending);

		assert_eq!(sorted.len(), list.len());
		check_asc(sorted);
	}

	#[test]
	fn can_sort_descending() {
		let list = Vec::from(UNSORTED_LIST);
		let sorted = merge_sort(&list, &Order::Descending);

		assert_eq!(sorted.len(), list.len());
		check_desc(sorted);
	}

	#[test]
	fn can_sort_via_trait() {
		let list = Vec::from(UNSORTED_LIST);
		let sorted = list.merge_sort(Order::Ascending);

		assert_eq!(sorted.len(), list.len());
		check_asc(sorted);
	}
}
