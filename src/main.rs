mod content;
mod error;
mod config;

use config::Config;

use rouille::Request;
use rouille::Response;
use rouille::router;
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

fn server(request: &Request) -> Response {
	router!{ request,
		(GET) (/) 		=> { routes::index() },
		(GET) (/media) 	=> { routes::media() },
		_ 				=> { rouille::match_assets(request, "./public") },
	}
}

fn main() {
	let config = config::get_config("Config.toml");

	let listen_address = format!("{}:{}", config.bind_address, config.port);
	rouille::start_server(&listen_address, server);
}
