#![allow(dead_code)]
#![allow(unused)] 

fn main() {
	let num: u32 = 516898014;
	let col: u8  = 6;
	let row: u8  = 5;
	
	for y in 0..row {
		for x in 0..col {
			if ((num >> (row - y - 1) * col + x) & 1) != 0 {
				print!("*");
			} else {
				print!(" ");
			}
		}
		println!("");
	}
	/*
	*/
}

// vim:ft=rust:noet:ai:cin:nosi

