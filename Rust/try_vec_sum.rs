#![allow(dead_code)]
#![allow(unused)]

fn main() {
	let vector: Vec<i32> = vec![2, 1, 4, 3, 7];
	let target: i32 = 10;
	let mut result_1: Vec<i32> = Vec::new();
	
	/* task 1:
	 *  get the index number of values from "vector" that the sum
	 *  of each values will equal to the "target" value then store
	 *  that index number into the "result" vector.
	 *
	 * example:
	 *  vector is [3, 5, 2, 8] 
	 *  target is 10
	 *  result is [0, 1, 2]
	 * 
	 * explanation:
	 *   0 =>  3
	 *   1 =>  5
	 *   2 =>  2
	 *       ---- +
	 *        10
	 */
	
	/* solution for task 1 */
	
	/* task 2:
	 *   same as before, but this time you have to find all 
	 *   possibilities and add '-1' as delimiter among them
	 *
	 * example:
	 *  vector is [3, 5, 2, 8] 
	 *  target is 10
	 *  result is [0, 1, 2, -1, 2, 3, -1]
	 * 
	 * explanation:
	 *   0 =>  3
	 *   1 =>  5
	 *   2 =>  2
	 *       ---- +
	 *        10
	 *  -1 => delimiter
	 *   2 =>  2
	 *   3 =>  8
	 *       ---- +
	 *        10
	 *  -1 => delimiter
	 */
	
	let mut result_2: Vec<i32> = Vec::new();
	
	/* solution for task 2 */
	
	/* check the "result" vector */
	assert_eq!(&[0, 1, 2, 3], &result_1[..]);
	assert_eq!(&[0, 1, 2, 3, -1, 3, 4, -1], &result_2[..]);
}

// vim:ft=rust:noet:ai:cin:nosi

