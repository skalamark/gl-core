// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::position::Position;
use anyhow;

pub type AnyError = anyhow::Error;

#[derive(Debug)]
pub struct Exception {
	pub module: String,
	pub position: Position,
	pub error: Error,
}

#[derive(Debug)]
pub enum ExceptionType {
	UnexpectedEOF,
	InvalidSyntax,
	Name,
}

#[derive(Debug)]
pub struct Error {
	pub class: ExceptionType,
	pub message: String,
}

impl std::fmt::Display for Exception {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		writeln!(
			f,
			"  File \"{}\", line {} column {}",
			&self.module,
			self.position.line + 1,
			self.position.column
		)?;
		write!(f, "{}", &self.error)
	}
}

impl std::fmt::Display for ExceptionType {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			ExceptionType::UnexpectedEOF => write!(f, "UnexpectedEOF"),
			ExceptionType::InvalidSyntax => write!(f, "InvalidSyntax"),
			ExceptionType::Name => write!(f, "NameError"),
		}
	}
}

impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}: {}", &self.class, &self.message)
	}
}

impl std::error::Error for Exception {}

impl Exception {
	pub fn new(module: String, position: Position, error: Error) -> AnyError {
		Self {
			module,
			position,
			error,
		}
		.into()
	}
}

impl Error {
	pub fn custom(class: ExceptionType, message: String) -> Self {
		Error { class, message }
	}

	pub fn unexpected_eof(message: String) -> Self {
		Self::custom(ExceptionType::UnexpectedEOF, message)
	}

	pub fn invalid_syntax(message: String) -> Self {
		Self::custom(ExceptionType::InvalidSyntax, message)
	}

	pub fn name(message: String) -> Self {
		Self::custom(ExceptionType::Name, message)
	}
}
