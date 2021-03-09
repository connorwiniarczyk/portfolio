/// A dedicated file for custom error types

use std::io;
use toml;


/// Used to represent errors from reading a toml file deserializing it into a
/// custom data structure. Is a union of the io::Error and toml::de::Error types
#[derive(Debug)]
pub enum ReadError {
	IoError(io::Error),
	ParseError(toml::de::Error),
}

impl From<io::Error> for ReadError {
	fn from(error: io::Error) -> Self {
		ReadError::IoError(error)
	}
}

impl From<toml::de::Error> for ReadError {
	fn from(error: toml::de::Error ) -> Self {
		ReadError::ParseError(error)
	}
}

// impl Display for ReadError {
// }
