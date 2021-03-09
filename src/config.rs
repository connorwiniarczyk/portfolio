/// Parses a config.toml file into a config data structure

use serde::{Serialize, Deserialize};

use std::default::Default;

use crate::error::ReadError;

pub struct Config {
	pub port: u32,
	pub bind_address: String,
}

impl From<FallibleConfig> for Config {
	fn from(config: FallibleConfig) -> Self {

		let default = Self::default();

		let port = match config.port {
			Some(port) => port,
			None => default.port,
		};

		let bind_address = match config.bind_address {
			Some(bind_address) => bind_address,
			None => default.bind_address
		};

		Self {port, bind_address}
	}
}

impl Default for Config {
	fn default() -> Self {
		Config {
			port: 3000,
			bind_address: "0.0.0.0".to_string(),
		}
	}
}

#[derive(Deserialize)]
struct FallibleConfig {
	pub port: Option<u32>,
	pub bind_address: Option<String>,
}

impl FallibleConfig {
	pub fn from_toml_file(path: &str) -> Result<Self, ReadError> {
		let file_contents = std::fs::read_to_string(path)?;		
		let data: Self = toml::from_str(&file_contents)?;
		Ok(data)
	}
}


pub fn get_config(path: &str) -> Config {	
	let result = FallibleConfig::from_toml_file(path);

	match result {
		Ok(fallible_config) => Config::from(fallible_config),
		Err(err) => {
			println!("oh no an error!");
			Config::default()
		}
	}
}
