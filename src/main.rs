#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::json::Json;
use rocket::response::NamedFile;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Media {
	pub url: String,
    pub media_type: String,
	pub id: Option<String>,
	pub title: Option<String>,
    pub description: Option<String>,
    pub section: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Content {
    pub media: Vec<Media>,
}

#[get("/")]
fn index() -> Option<NamedFile> {
    NamedFile::open("/usr/public/index.html").ok()
}

#[get("/media")]
fn media_data() -> Option<Json<Content>> {
    let file = std::fs::read_to_string("Media.toml").unwrap();
    let data: Content = toml::from_str(&file).unwrap();

    Some(Json(data))
}

fn main() {
	rocket::ignite()
		.mount("/", routes![index, media_data])
		.mount("/stylesheets", StaticFiles::from("/usr/public/stylesheets"))
		.mount("/scripts", StaticFiles::from("/usr/public/scripts"))
		.mount("/fonts", StaticFiles::from("/usr/public/fonts"))
		.mount("/media", StaticFiles::from("/usr/media"))
		.launch();
}
