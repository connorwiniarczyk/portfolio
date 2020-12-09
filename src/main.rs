#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate tera;
mod content_parser;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> Template {
    let context = content_parser::get_content();
	Template::render("index", &context)
}

fn main() {
	rocket::ignite()
		.attach(Template::fairing())
		.mount("/", routes![index])
		.mount("/stylesheets", StaticFiles::from("./public/stylesheets"))
		.mount("/fonts", StaticFiles::from("./public/fonts"))
		.mount("/media", StaticFiles::from("../../Shared/portfolio_media"))
		.launch();
}
