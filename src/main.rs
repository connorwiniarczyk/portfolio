mod content;
mod error;

use rouille::Request;
use rouille::Response;
use crate::content::Content;
use std::fs::File;

mod routes {
	use crate::content::Content;
	use std::fs::File;
	use rouille::Response;

	/// route for /media
	/// returns json object with the contents of the Media.toml file
	pub fn media() -> Response {
		match Content::from_toml_file("Media.toml") {
			Ok(content) => Response::json(&content),
			Err(error) => {
				println!("error reading Media.toml:");
				println!("{:?}", error);
				Response::empty_404()
			}
		}
	}
	
	/// route for /
	/// returns the index.html page
	pub fn index() -> Response {
		match File::open("public/index.html") {
			Ok(file) => Response::from_file("text/html", file),
			Err(error) => {
				println!("error reading index.html:");
				println!("{:?}", error);
				Response::empty_404()
			}
	
		}
	}
}


use rouille::router;
fn server(request: &Request) -> Response {
	router!{ request,
		(GET) (/) 		=> { routes::index() },
		(GET) (/media) 	=> { routes::media() },
		_ 				=> { rouille::match_assets(request, "./public") },
	}
}

fn main() {

	let port = 8000;
	let host = "0.0.0.0";

	let listen_address = format!("{}:{}", host, port);
	rouille::start_server(&listen_address, server);
}
