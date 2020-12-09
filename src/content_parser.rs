use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Media {
	pub url: String,
    pub media_type: String,
	pub id: Option<String>,
	pub title: Option<String>,
    pub description: Option<String>,
    pub section: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Content {
	pub media: Vec<Media>,
}

impl Default for Content {
	fn default () -> Content {
		Content {
			media: Vec::new(),
		}
	}
}

pub fn get_content () -> Content {
	let file = fs::read_to_string("Media.toml").expect("error"); 
	let info: std::result::Result<Content, toml::de::Error> = toml::from_str(&file);
	let result = match info {
		Ok(value) => value,
		Err(e) => {
			println!("Error parsing content.toml: {}", e);
			Content::default()
		}
	};
	return result
}
