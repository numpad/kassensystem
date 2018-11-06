extern crate sqlite3 as sql;
use db::user::{User, UserType};
use std::env;

pub struct Datenbank {
	conn: sql::Connection,
}

impl Datenbank {
	pub fn connect() -> Datenbank {
		let db = Datenbank {
			conn: sql::open(path_to_data("getraenke.db")).expect("could not open db"),
		};

		db.conn.execute(
			"CREATE TABLE IF NOT EXISTS konten(name TEXT, balance INT, utype TEXT, last_active INT, deleted INT);"
		).expect("could not create table 'konten'");

		db
	}

	pub fn add_user(&self, user: &User) {
		let result = self.conn.execute(
			format!(
				"INSERT INTO konten(name, balance, utype, last_active, deleted) VALUES(\"{}\", {}, \"{}\", {}, 0);",
				user.name(),
				user.balance(),
				user.utype(),
				user.last_active()
			)
		);

		if let Err(e) = result {
			eprintln!("could not execute user-add: {}", e);
		}
	}

	pub fn delete_user(&self, user_id: i64) -> Option<()> {
		let result = self.conn.execute(
			format!("UPDATE konten SET deleted = 1 WHERE rowid = {}", user_id)
		);

		if let Err(e) = result {
			eprintln!("Could not delete user {}, error: {}", user_id, e);
			return None
		}

		Some(())
	}

	pub fn update_user(&self, user: &User) -> Option<()> {
		let result = self.conn.execute(
			format!("UPDATE konten SET name='{}',balance={},utype='{}' WHERE rowid = {}", user.name, user.balance, user.utype, user.rowid?)
		);
		
		if let Err(e) = result {
			eprintln!("Could not update user {}, error: {}", user.rowid?, e);
			return None
		}

		Some(())
	}

	pub fn get_users(&self) -> Vec<User> {
		let mut users = vec![];

		let mut statement = self.conn.prepare("SELECT name, balance, utype, last_active, deleted, rowid FROM konten;").expect("could not prepare statement");
		while let sql::State::Row = statement.next().unwrap() {
			let user = User {
				name: statement.read::<String>(0).unwrap(),
				balance: statement.read::<i64>(1).unwrap() as i32,
				utype: UserType::from(statement.read::<String>(2).unwrap().as_str()),
				last_active: 0,
				rowid: Some(statement.read::<i64>(5).unwrap() as u32),
				deleted: statement.read::<i64>(4).unwrap() as i32,
			};

			if !user.deleted() {
				users.push(user);
			}
		}

		users
	}
}

fn path_to_data(file: &str) -> String {
	let path = env::current_dir().unwrap();
	path.join("res").join("data").join(file).to_str().unwrap().to_owned()
}
