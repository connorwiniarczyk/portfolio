/// A data structure representing the contents of the Media.toml file

use serde::{Serialize, Deserialize};
use rouille::Response;

use crate::error::ReadError;

#[derive(Serialize, Deserialize)]
pub struct Media {
	pub url: String,
    pub media_type: String,
	pub id: Option<String>,
	pub title: Option<String>,
    pub description: Option<String>,
    pub section: Option<String>,
}

use std::io;

#[derive(Serialize, Deserialize)]
pub struct Content {
    pub media: Vec<Media>,
}


use serde::de::Error;
impl Content {
	pub fn from_toml_file(path: &str) -> Result<Self, ReadError> {
		let file_contents = std::fs::read_to_string(path)?;		
		let data: Self = toml::from_str(&file_contents)?;
		Ok(data)
	}
}

