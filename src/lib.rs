#![warn(missing_docs)]
//! This crate was mainly made so I could learn rust by doing something simple and properly integrating it.
//! It contains all kinds of sorting algorithms that can be used to sort anything iterable.

pub use crate::bubble_sort::{BubbleSortable, bubble_sort};
pub use crate::insertion_sort::{InsertionSortable, insertion_sort};
pub use crate::merge_sort::{MergeSortable, merge_sort};
pub use crate::selection_sort::{SelectionSortable, selection_sort};

mod bubble_sort;
mod insertion_sort;
mod merge_sort;
mod selection_sort;

/// The ordering to be used by all sorting algorithms.
pub enum Order {
	/// Sort in ascending order.
	Ascending,
	/// Sort in descending order.
	Descending,
}

#[cfg(test)]
mod testing;
