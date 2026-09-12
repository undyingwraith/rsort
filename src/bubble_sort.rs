use crate::Difference;

pub fn bubble_sort<T: Clone + Copy>(input: &[T], compare: fn(a: T, b: T) -> Difference) -> Vec<T> {
	let mut sorted = false;
	let mut list = input.to_vec();
	let len = input.len();
	while !sorted {
		sorted = true;
		for i in 0..(len - 1) {
			match compare(list[i], list[i + 1]) {
				Difference::Larger => {
					let t = list[i];
					list[i] = list[i + 1];
					list[i + 1] = t;
					sorted = false;
				}
				_ => {}
			}
		}
	}

	list
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn can_sort() {
		let list = Vec::from([4, 3, 5, 1, 6, 2, 7, 9, 8]);
		let sorted = bubble_sort(&list, |a, b| (a - b).into());

		assert_eq!(sorted.len(), list.len());
		assert_eq!(sorted[0], 1);
		assert_eq!(sorted[1], 2);
		assert_eq!(sorted[2], 3);
		assert_eq!(sorted[3], 4);
		assert_eq!(sorted[4], 5);
		assert_eq!(sorted[5], 6);
		assert_eq!(sorted[6], 7);
		assert_eq!(sorted[7], 8);
		assert_eq!(sorted[8], 9);
	}
}
