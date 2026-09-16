pub use crate::bubble_sort::bubble_sort;
pub use crate::selection_sort::selection_sort;

mod bubble_sort;
mod selection_sort;

pub enum Difference {
	Larger,
	Smaller,
	Same,
}

impl From<i32> for Difference {
	fn from(i: i32) -> Self {
		if i > 0 {
			Difference::Larger
		} else if i < 0 {
			Difference::Smaller
		} else {
			Difference::Same
		}
	}
}
