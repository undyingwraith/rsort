pub use crate::bubble_sort::{BubbleSortable, bubble_sort};
pub use crate::insertion_sort::{InsertionSortable, insertion_sort};
pub use crate::selection_sort::{SelectionSortable, selection_sort};

mod bubble_sort;
mod insertion_sort;
mod selection_sort;

pub enum Order {
	Ascending,
	Descending,
}

#[cfg(test)]
mod testing;
