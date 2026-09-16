pub const UNSORTED_LIST: [i32; 9] = [4, 3, 5, 1, 6, 2, 7, 9, 8];

pub fn check_asc(sorted: Vec<i32>) {
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

pub fn check_desc(sorted: Vec<i32>) {
	assert_eq!(sorted[0], 9);
	assert_eq!(sorted[1], 8);
	assert_eq!(sorted[2], 7);
	assert_eq!(sorted[3], 6);
	assert_eq!(sorted[4], 5);
	assert_eq!(sorted[5], 4);
	assert_eq!(sorted[6], 3);
	assert_eq!(sorted[7], 2);
	assert_eq!(sorted[8], 1);
}
