extern crate tiny_http;
extern crate tera;
extern crate url;
use std::env;
use std::fs::File;
use self::url::Url;
use std::path::PathBuf;

fn static_path() -> String {
	let path = env::current_dir().unwrap();
	path.join("res").join("static").to_str().unwrap().to_owned()
}

fn templates_path() -> String {
	let path = env::current_dir().unwrap();
	path.join("res").join("templates").join("**").join("*").to_str().unwrap().to_owned()
}

fn send_response(tera: &mut tera::Tera, context: &tera::Context, request: tiny_http::Request, template: &str) {
	let response = tera.render(template, context).unwrap();
	let response = tiny_http::Response::from_data(response);
	request.respond(response).expect("could not respond");
}

pub fn run(port: u16) {
	let server = tiny_http::Server::http(format!("localhost:{}", port)).expect("could not run server!");
	let mut tera = compile_templates!(&templates_path());
	tera.autoescape_on(vec!["html"]);

	loop {
		/* receive and handle request */
		let request = match server.recv() {
			Ok(req) => req,
			Err(err) => { eprintln!("could not recv: {}", err); break; }
		};
		tera.full_reload().expect("could not full-reload");
		let path = Url::parse(&("http://0.0.0.0".to_owned() + request.url())).expect("could not parse url");
		let path = path.path_segments().map(|c| c.collect::<Vec<_>>() ).unwrap_or(vec![]);
		println!(">> {:?}", path);
		let mut context = tera::Context::new();

		println!("path-len: {}", path.len());
		if path.len() == 1 && path[0] == "" {
			context.insert("pages", &vec![
				("/LF10.html", "/LF10/ Fs-Mitglied"),
				("/LF20.html", "/LF20/ Kassenwart"),
				("/LF30.html", "/LF30/ Getränkewart"),
				("/LF40.html", "/LF40/ System"),
			]);

			send_response(&mut tera, &context, request, "index.html");
		} else if path.len() == 2 && path[0] == "static" {
			println!("<<{:?}", PathBuf::from(static_path()).join(path[1]));
			let response = tiny_http::Response::from_file(File::open(
				PathBuf::from(static_path()).join(path[1])
			).unwrap());
			request.respond(response).expect("could not respond");
		} else {
			send_response(&mut tera, &context, request, "404.html");
		}
	}
}
