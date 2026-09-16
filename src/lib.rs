pub use crate::bubble_sort::bubble_sort;
pub use crate::selection_sort::selection_sort;

mod bubble_sort;
mod selection_sort;

pub enum Order {
	Ascending,
	Descending,
}

#[cfg(test)]
mod testing;
