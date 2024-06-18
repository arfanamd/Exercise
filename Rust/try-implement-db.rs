#![allow(dead_code)]
#![allow(unused)]

use std::{
	fmt,
	io,
	fs,
};

struct Employee {
	id: usize,
	set: bool,
	mail: String,
}

impl Employee {
	fn new(id: &usize) -> Self {
		return Employee {
			id: *id,
			set: false,
			mail: String::new(),
		};
	}
}

/* impl fmt::Debug for Employee { *{{{1
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return write!(f, "\"{}\",\"{}\",\"{}\"", self.mail, self.id, self.set);
	}
} *}}} */

const MAX_ROWS: usize = 2;

struct Database {
	rows: Vec<Employee>,
}

impl Database {
	fn init() -> Self {
		let mut db = Database {
			rows: Vec::with_capacity(MAX_ROWS),
		};
		
		for data in 0..MAX_ROWS {
			db.rows.push(Employee::new(&data));
		}
		
		return db;
	}
	
	fn print(&self, id: usize) {
		if self.is_set(&id) {
			println!("\"{}\",\"{}\"", self.rows[id].mail, id);
		} else {
			println!("address {} is empty", id);
		}
	}
	
	fn is_set(&self, id: &usize) -> bool {
		return self.rows[*id].set;
	}
	
	fn set(&mut self, mail: String) {
		for db in self.rows.iter_mut().enumerate() {
			let (id, data): (usize, &mut Employee) = db;
			
			/* TODO: there is a bug inside this logic */
			if id < MAX_ROWS {
				println!("set for address {}", id);
				if data.set == false {
					data.set = true;
					data.mail = mail.clone();
					break;
				}
			} else {
				panic!("database is full");
			}
		}
	}
	
	fn unset(&mut self, id: usize) {
		if id < MAX_ROWS {
			if self.is_set(&id) {
				self.rows[id] = Employee::new(&id);
			} else {
				panic!("address {} is not set", id);
			}
		} else {
			panic!("address {} is out of scope", id);
		}
	}
}

/* TODO: Find a way to store Database data into a db file! {{{1
struct Connection {
	db: Database,
	file: fs::File,
}

impl Connection {
	fn db_create(&mut self, db_path: &String) -> io::Result<()> {
		self.file = fs::File::create(db_path)?;
		self.db.init();

		/* This error confuse me.. Damn
		self.file.write_all(self.db.rows[i] = Employee {
			id: i as u32,
			set: false,
			mail: Default::default(),
		})?;
		*/

		return Ok(());
	}

	fn db_open(&mut self, db_path: &String) -> io::Result<()> {
		self.file = fs::File::open(db_path)?;
		Ok(())
	}
}
* }}} */

fn main() {
	let mut my_db = Database::init();
	my_db.set("cat@domain.my".to_string());
	my_db.set("dog@domain.my".to_string());
	my_db.set("eel@domain.my".to_string());
	my_db.set("fish@domain.my".to_string());

	my_db.print(0);
	my_db.print(1);
	my_db.print(2);
	my_db.print(3);
}

// vim:ft=rust:noet:ai:cin:nosi
