#![allow(dead_code)]
#![allow(unused)]

const OPTFLAG_MSG_INVALID: &str = "This flag isn't valid";
const OPTFLAG_MSG_MISSING: &str = "Flag need an argument";
const OPTFLAG_MSG_TOOMANY: &str = "Flag need no argument";

enum OptFlagTypes {
	OptFlagTypeNoop,
	OptFlagTypeOptional(Option<String>),
	OptFlagTypeMandatory(String),
}

struct OptFlagProp {
	args: Vec<String>,
	permute: bool,
	flagidx: i32,
	flagopt: i32,
	flagarg: String,
	flagerr: String,
	flagsub: i32,
}

impl Default for OptFlagProp {
	fn default() -> Self {
		return Self {
			args: Vec::new(),
			permute: true,
			flagidx: 1,
			flagopt: 0,
			flagarg: String::from(""),
			flagerr: String::from(""),
			flagsub: 0,
		};
	}
}

impl OptFlagProp {
	fn init(args: Vec<String>) -> Self {
		return Self::default();
	}
	fn next(&self, flag: String) -> Option<i32> {
		return None;
	}
	fn arg(&self) -> String {
		return String::from("");
	}
}

// fn long()

fn main() {
	println!("Hello, world!");
}

// vim:ft=rust:noet:ai:cin:nosi

