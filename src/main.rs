#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate tera;

mod content_parser;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use comrak::{markdown_to_html, ComrakOptions};

use std::collections::HashMap;

use std::fs;

#[get("/")]
fn index() -> Template {
    let context = content_parser::get_content();
	Template::render("index", &context)
}

#[get("/projects/<page>")]
fn page(page: String) -> Template{
    let filepath = format!("./public/pages/{}.md", &page);
    let markdown = fs::read_to_string(&filepath).expect("");
    let html = markdown_to_html(&markdown, &ComrakOptions::default());

    let mut context = HashMap::new();
    context.insert("body".to_string(), &html);

    Template::render("project", &context)
}

fn main() {
    let md = fs::read_to_string("./public/pages/bms.md").expect("error");
    println!("{}", markdown_to_html(&md, &ComrakOptions::default()));

	rocket::ignite()
		.attach(Template::fairing())
		.mount("/", routes![index, page])
		.mount("/stylesheets", StaticFiles::from("./public/stylesheets"))
		.mount("/scripts", StaticFiles::from("./public/scripts"))
		.mount("/fonts", StaticFiles::from("./public/fonts"))
		.mount("/media", StaticFiles::from("../../Shared/portfolio_media"))
		.launch();
}
