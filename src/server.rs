extern crate tiny_http;

pub fn run(port: u16) {
	let server = tiny_http::Server::http(format!("localhost:{}", port)).expect("could not run server!");

	loop {
		/* receive and handle request */
		let request = match server.recv() {
			Ok(req) => req,
			Err(err) => { eprintln!("recv-error: {}", err); break; }
		};

		let response = tiny_http::Response::from_data();
		request.respond(response);
	}
}
