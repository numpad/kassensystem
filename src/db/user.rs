use std::fmt::{Display, Formatter, Error};
extern crate serde;

#[derive(Serialize, Deserialize)]
pub enum UserType {
	Normal,
	Unlimited,
}

impl Display for UserType {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
		let utypename = match self {
			UserType::Normal	=> "Normal",
			UserType::Unlimited	=> "Unlimited",
		};

		write!(fmt, "{}", utypename)
	} 
}

impl<'a> From<&'a str> for UserType {
	fn from(input: &str) -> UserType {
		match input {
			"Normal" => UserType::Normal,
			"Unlimited" => UserType::Unlimited,
			_ => UserType::Normal,
		}
	}
}

#[derive(Serialize, Deserialize)]
pub struct User {
	pub name: String,
	pub balance: i32,
	pub utype: UserType,
	pub last_active: u32,
	pub rowid: Option<u32>,
	pub deleted: i32,
}

impl User {
	pub fn name(&self) -> &str {
		&self.name
	}
	pub fn balance(&self) -> i32 {
		self.balance
	}
	pub fn utype(&self) -> &UserType {
		&self.utype
	}
	pub fn last_active(&self) -> u32 {
		self.last_active
	}
	pub fn rowid(&self) -> Option<u32> {
		self.rowid
	}
	pub fn deleted(&self) -> bool {
		self.deleted > 0
	}
}
