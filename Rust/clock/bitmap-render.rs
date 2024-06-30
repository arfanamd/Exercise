#![allow(dead_code)]
#![allow(unused)] 

fn render(num: &u32) {
	const COL: u8 = 6;
	const ROW: u8 = 5;
	
	for y in 0..ROW {
		for x in 0..COL {
			if ((num >> (ROW - y - 1) * COL + x) & 1) != 0 {
				print!("*");
			} else {
				print!(" ");
			}
		}
		println!("");
	}
}
fn conv(n: &u32) {
	const BYTES: u8 = 30;
	const BITS:  u8 = 6;
	
	let mask: u32 = 1 << (BYTES - 1);
	let mut n: u32 = *n;
	
	for i in 0..BYTES {
		match n & mask {
			0 => print!(" "),
			_ => print!("*"),
		}
		n <<= 1;
		
		if (i + 1) % BITS == 0 {
			println!("");
		}
	}
}

fn main() {
	let map: [u32; 04] = [516898014, 1009828671, 516711231, 516717790];

	for i in 0..4 {
		render(&map[i]);
		conv(&map[i]);
		println!("");
	}
	/*
	*/
}

// vim:ft=rust:noet:ai:cin:nosi

