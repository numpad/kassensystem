extern crate kassensystem;
use kassensystem::server;
use kassensystem::db::db::Datenbank;

fn main() {
	let port = 8080;

	println!("starting server on port {}...", port);
	let mut db = Datenbank::connect();
	server::run(port, &mut db);
}
