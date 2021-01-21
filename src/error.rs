// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::position::Position;
use anyhow;

pub type AnyError = anyhow::Error;

#[derive(Debug)]
pub struct ExceptionMain {
	pub exceptions: Vec<Exception>,
	pub error: ExceptionError,
	pub is_runtime: bool,
}

#[derive(Debug)]
pub struct Exception {
	pub module: String,
	pub position: Position,
}

#[derive(Debug)]
pub struct ExceptionError {
	pub class: ExceptionType,
	pub message: String,
}

#[derive(Debug)]
pub enum ExceptionType {
	UnexpectedEOF,
	InvalidSyntax,
	Name,
	Type,
}

impl std::fmt::Display for ExceptionMain {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		if self.is_runtime {
			writeln!(f, "Traceback (most recent call last):")?;
		}

		for exception in self.exceptions.iter() {
			if self.is_runtime {
				writeln!(
					f,
					"  File \"{}\", line {}",
					&exception.module,
					exception.position.line + 1,
				)?;
			} else {
				writeln!(
					f,
					"  File \"{}\", line {} column {}",
					&exception.module,
					exception.position.line + 1,
					exception.position.column + 1
				)?;
			}
		}

		write!(f, "{}", &self.error)
	}
}

impl std::fmt::Display for ExceptionError {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}: {}", &self.class, &self.message)
	}
}

impl std::fmt::Display for ExceptionType {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			ExceptionType::UnexpectedEOF => write!(f, "UnexpectedEOF"),
			ExceptionType::InvalidSyntax => write!(f, "InvalidSyntax"),
			ExceptionType::Name => write!(f, "NameError"),
			ExceptionType::Type => write!(f, "TypeError"),
		}
	}
}

// impl std::error::Error for Exception {}

impl ExceptionMain {
	pub fn new(error: ExceptionError, is_runtime: bool) -> Self {
		Self {
			exceptions: Vec::new(),
			error,
			is_runtime,
		}
	}

	pub fn push(&mut self, exception: Exception) {
		self.exceptions.push(exception);
	}
}

impl Exception {
	pub fn new(module: String, position: Position) -> Self {
		Self { module, position }
	}
}

impl ExceptionError {
	pub fn custom(class: ExceptionType, message: String) -> Self {
		ExceptionError { class, message }
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

	pub fn type_(message: String) -> Self {
		Self::custom(ExceptionType::Type, message)
	}
}
