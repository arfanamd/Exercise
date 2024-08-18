#![allow(dead_code)]
#![allow(unused)] 

fn parse(arg: &[u8]) -> Option<usize> {
	let mut slice = arg.iter().enumerate().peekable();
	while let Some((index, &character)) = slice.next() {
		if Some(&character) == Some(&b'-') {
			if let Some((_, &b'-')) = slice.peek() {
				return Some(index);
			}
		}
	}
	return None;
}

fn extr_flag<'flag>(opt: &'flag str) { //-> char {
	let mut ch = opt.chars().peekable();

	if let Some(a) = ch.next_if_eq(&'-') {
		if let Some(b) = ch.next_if_eq(&'-') {
			if let Some(c) = ch.peek() {
				println!("is long flag {c}");
			} else {
				println!("is dash-dash");
			}
		} else if let Some(c) = ch.next() {
			println!("is a short flag {c}");
		} else {
			println!("not valid");
		}
	} else {
		println!("not a flag");
	}
}

enum OptFlagTypes {
	Noop,
	Optional,
	Mandatory,
}

struct OptFlag {
	flag: OptFlagTypes,
}

fn optflag(argv: &Vec<&str>, args: &str) -> Option<char> {
	return None;
}

fn main() {
	let s: Vec<&str> = vec!["--option", "bazz", "-foo", "--", "floo", "bar"];
	let f: &str = "f:";

	println!("{:?}", optflag(&s, f));
}

// vim:ft=rust:noet:ai:cin:nosi
