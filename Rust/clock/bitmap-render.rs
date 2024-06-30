#![allow(dead_code)]
#![allow(unused)] 

fn render(col: &u8, row: &u8, num: &u32) {
	for y in 0..*row {
		for x in 0..*col {
			if ((num >> (row - y - 1) * col + x) & 1) != 0 {
				print!("*");
			} else {
				print!(" ");
			}
		}
		println!("");
	}
}

fn main() {
	let col: u8        = 6;
	let row: u8        = 5;
	let map: [u32; 04] = [516898014, 1009828671, 516711231, 516717790];

	for i in 0..4 {
		render(&col, &row, &map[i]);
		println!("");
	}
	/*
	*/
}

// vim:ft=rust:noet:ai:cin:nosi

