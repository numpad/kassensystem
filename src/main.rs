extern crate kassensystem;
use kassensystem::server;
use kassensystem::db::db::Datenbank;

fn main() {
	println!("start");
	let mut db = Datenbank::connect();
	server::run(8080, &mut db);
}
