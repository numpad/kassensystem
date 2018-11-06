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
			"CREATE TABLE IF NOT EXISTS konten(name TEXT, balance INT, utype TEXT, last_active INT);"
		).expect("could not create table 'konten'");

		db
	}

	pub fn add_user(&self, user: &User) {
		self.conn.execute(
			format!(
				"INSERT INTO konten(name, balance, utype, last_active) VALUES({}, {}, {}, {});",
				user.name(),
				user.balance(),
				user.utype(),
				user.last_active()
			)
		).expect("could not execute user-add");
	}

	pub fn read_all_users(&self) -> Vec<User> {
		let mut users = vec![];

		let mut statement = self.conn.prepare("SELECT * FROM konten;").expect("could not prepare statement");
		while let sql::State::Row = statement.next().unwrap() {
			let user = User {
				name: statement.read::<String>(0).unwrap(),
				balance: statement.read::<i64>(1).unwrap() as i32,
				utype: UserType::from(""),
				last_active: 0,
			};

			users.push(user);
		}

		users
	}
}

fn path_to_data(file: &str) -> String {
	let path = env::current_dir().unwrap();
	path.join("res").join("data").join(file).to_str().unwrap().to_owned()
}
