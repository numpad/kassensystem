use std::fmt::{Display, Formatter, Error};
extern crate serde;
use db::user::serde::ser::Serialize;

#[derive(Serialize, Deserialize)]
pub enum UserType {
	Normal,
	Unlimited,
}

impl Display for UserType {
	fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
		let utypename = match self {
			UserType::Normal	=> "FS-Mitglied",
			UserType::Unlimited	=> "Guthabenkonto",
		};

		write!(fmt, "{}", utypename)
	} 
}

impl<'a> From<&'a str> for UserType {
	fn from(input: &str) -> UserType {
		match input {
			"FS-Mitglied" => UserType::Normal,
			"Guthabenkonto" => UserType::Unlimited,
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
}
