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
			mail: Default::default(),
		};
	}
	
	fn get(&self) -> &String {
		return &self.mail;
	}
}

impl fmt::Debug for Employee {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		return write!(f, "\"{}\",\"{}\",\"{}\"", self.mail, self.id, self.set);
	}
}

const MAX_ROWS: usize = 32;

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
		println!("\"{}\",\"{}\"", self.rows[id].get(), id);
	}
	
	fn is_set(&self, id: &usize) -> bool {
		return self.rows[*id].set;
	}
	
	fn set(&mut self, id: usize, mail: String) -> Result<(), &'static str> {
		if id < MAX_ROWS {
			if self.is_set(&id) {
				return Err("Already set, remove it first!");
			}
			
			self.rows[id].set = true;
			self.rows[id].mail = mail;
			return Ok(());
		}
		
		return Err("Id is out of scope");
	}

	fn unset(&mut self, id: usize) -> Result<(), &'static str> {
		if id < MAX_ROWS {
			if self.is_set(&id) {
				return Err("It is not yet set");
			}
			
			self.rows[id] = Employee::new(&id);
			return Ok(());
		}
		
		return Err("It is not yet set");
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
	my_db.set(0, "cat@domain.my".to_string());
	my_db.print(0);
	println!("{:?}", my_db.rows[0]);
}

// vim:ft=rust:noet:ai:cin:nosi
