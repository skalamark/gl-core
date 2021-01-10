// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use anyhow;

#[derive(Debug)]
pub struct Error {
	pub class: String,
	pub message: String,
}

pub type AnyError = anyhow::Error;

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}: {}", &self.class, &self.message)
	}
}

impl std::error::Error for Error {}

impl Error {
	pub fn custom(class: &str, message: String) -> AnyError {
		Error {
			class: format!("{}", class),
			message,
		}
		.into()
	}

	pub fn type_(message: String) -> AnyError {
		Self::custom("TypeError", message)
	}

	pub fn invalid_syntax(message: String) -> AnyError {
		Self::custom("InvalidSyntax", message)
	}
}
