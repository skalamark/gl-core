// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

#[derive(Debug)]
pub struct Exception {
	pub exception_points: Vec<ExceptionPoint>,
	pub except: Except,
	pub is_runtime: bool,
}

#[derive(Debug)]
pub struct ExceptionPoint {
	pub module_context: String,
	pub position: Position,
}

#[derive(Debug)]
pub struct Except {
	pub typer: ExceptType,
	pub message: String,
}

#[derive(Debug)]
pub enum ExceptType {
	AttributeError,
	Eof,
	Error,
	Import,
	Index,
	InvalidSyntax,
	Key,
	KeyboardInterrupt,
	Name,
	Type,
	UnexpectedEOF,
}

impl std::fmt::Display for Exception {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		if self.is_runtime {
			writeln!(f, "Traceback (most recent call last):")?;
		}

		for exception_point in self.exception_points.iter() {
			if self.is_runtime {
				writeln!(
					f,
					"  File \"{}\", line {}",
					&exception_point.module_context,
					exception_point.position.line + 1,
				)?;
			} else {
				writeln!(
					f,
					"  File \"{}\", line {} column {}",
					&exception_point.module_context,
					exception_point.position.line + 1,
					exception_point.position.column + 1
				)?;
			}
		}

		write!(f, "{}", &self.except)
	}
}

impl std::fmt::Display for Except {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		if self.message.is_empty() {
			write!(f, "{}", &self.typer)
		} else {
			write!(f, "{}: {}", &self.typer, &self.message)
		}
	}
}

impl std::fmt::Display for ExceptType {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			ExceptType::AttributeError => write!(f, "AttributeError"),
			ExceptType::Eof => write!(f, "EOFError"),
			ExceptType::Error => write!(f, "Error"),
			ExceptType::Import => write!(f, "ImportError"),
			ExceptType::Index => write!(f, "IndexError"),
			ExceptType::InvalidSyntax => write!(f, "InvalidSyntax"),
			ExceptType::Key => write!(f, "KeyError"),
			ExceptType::KeyboardInterrupt => write!(f, "KeyboardInterrupt"),
			ExceptType::Name => write!(f, "NameError"),
			ExceptType::Type => write!(f, "TypeError"),
			ExceptType::UnexpectedEOF => write!(f, "UnexpectedEOF"),
		}
	}
}

impl Exception {
	fn new(except: Except, is_runtime: bool) -> Self {
		Self { exception_points: Vec::new(), except, is_runtime }
	}

	pub fn not_runtime(except: Except) -> Self { Self::new(except, false) }

	pub fn in_runtime(except: Except) -> Self { Self::new(except, true) }

	pub fn push(&mut self, exception_point: ExceptionPoint) {
		self.exception_points.push(exception_point);
	}
}

impl ExceptionPoint {
	pub fn new<T: Into<String>>(module_context: T, position: Position) -> Self {
		Self { module_context: module_context.into(), position }
	}
}

impl Except {
	fn new<T: Into<String>>(typer: ExceptType, message: T) -> Self {
		Except { typer, message: message.into() }
	}

	pub fn attribute<T: Into<String>>(message: T) -> Self {
		Self::new(ExceptType::AttributeError, message)
	}

	pub fn eof<T: Into<String>>(message: T) -> Self { Self::new(ExceptType::Eof, message) }

	pub fn error<T: Into<String>>(message: T) -> Self { Self::new(ExceptType::Error, message) }

	pub fn import<T: Into<String>>(message: T) -> Self { Self::new(ExceptType::Import, message) }

	pub fn invalid_syntax<T: Into<String>>(message: T) -> Self {
		Self::new(ExceptType::InvalidSyntax, message)
	}

	pub fn index<T: Into<String>>(message: T) -> Self { Self::new(ExceptType::Index, message) }

	pub fn key<T: Into<String>>(message: T) -> Self { Self::new(ExceptType::Key, message) }

	pub fn keyboard_interrupt<T: Into<String>>(message: T) -> Self {
		Self::new(ExceptType::KeyboardInterrupt, message)
	}

	pub fn name<T: Into<String>>(message: T) -> Self { Self::new(ExceptType::Name, message) }

	pub fn type_<T: Into<String>>(message: T) -> Self { Self::new(ExceptType::Type, message) }

	pub fn unexpected_eof<T: Into<String>>(message: T) -> Self {
		Self::new(ExceptType::UnexpectedEOF, message)
	}
}
