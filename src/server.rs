extern crate tiny_http;
extern crate tera;
extern crate url;
use std::env;
use std::fs::File;
use self::url::Url;
use std::path::PathBuf;
use db::db::Datenbank;
use std::collections::HashMap;
use std::borrow::Borrow;

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

fn parse_formbody(body: &str) -> HashMap<&str, &str> {
	let mut form_data = HashMap::new();

	body.split("&").into_iter()
		.for_each(|e| {
			let mut kv = e.split("=").into_iter();
			form_data.insert(
				kv.nth(0).expect(&format!("1) e = {}", e)),
				kv.nth(0).expect(&format!("2) e = {}", e))
			);
		});

	form_data
}

pub fn run(port: u16, db: &mut Datenbank) {
	let server = tiny_http::Server::http(format!("localhost:{}", port)).expect("could not run server!");
	let mut tera = compile_templates!(&templates_path());
	tera.autoescape_on(vec!["html"]);

	loop {
		/* receive and handle request */
		let mut request = match server.recv() {
			Ok(req) => req,
			Err(err) => { eprintln!("could not recv: {}", err); break; }
		};
		tera.full_reload().expect("could not full-reload");
		let path = Url::parse(&("http://0.0.0.0".to_owned() + request.url())).expect("could not parse url");
		let path = path.path_segments().map(|c| c.collect::<Vec<_>>() ).unwrap_or(vec![]);
		
		let mut context = tera::Context::new();

		// send response
		if path.len() == 2 && path[0] == "api" {
			let mut data = String::new();
			request.as_reader().read_to_string(&mut data);

			/*
			data.push_str("<hr>");
			for header in request.headers() {
				data.push_str(&format!("<p>{}: {}</p>", header.field.as_str(), header.value));
			}
			*/
			{
				let form = parse_formbody(&data);
				println!("{}", 
					form.get("name").unwrap()
				);
			}

			request.respond(tiny_http::Response::from_data(data)).unwrap();
		} else if path.len() == 1 && path[0] == "LF20.html" {
			let users = db.read_all_users();
			context.insert("users", &users);
			send_response(&mut tera, &context, request, "LF20.html");
		} else if path.len() == 1 && path[0] == "" {
			context.insert("pages", &vec![
				("/LF10.html", "Bediener", "Tabletmodus für Verwendung an der Theke. Hier vermerkt das Fachschaftsmitglied bestellte Getränke und führt Abbuchungen aus.", "info"),
				("/LF20.html", "Kassenwart", "Verwaltung der Kasse und des Guthabens. Hier verwaltet der Kassenwart Konten und Guthaben der Mitglieder.", "accent"),
				("/LF30.html", "Getränkewart", "Verwaltung des Getränkebestands.", "success"),
				("/LF40.html", "System", "Sonstige Systemeinstellungen und -funktionen.", "black"),
			]);

			send_response(&mut tera, &context, request, "index.html");
		} else if path.len() == 2 && path[0] == "static" {
			let response = tiny_http::Response::from_file(File::open(
				PathBuf::from(static_path()).join(path[1])
			).unwrap());
			request.respond(response).expect("could not respond");
		} else {
			send_response(&mut tera, &context, request, "404.html");
		}
	}
}

#[test]
fn test_parseform() {
	let form = parse_formbody("name=chris&balance=200%E2%82%AC&utype=Unlimited");

	assert_eq!(form.len(), 3);
}
