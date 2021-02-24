// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

pub type AnyError = anyhow::Error;

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
	Eof,
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
			ExceptType::Eof => write!(f, "EOFError"),
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
	pub fn new(except: Except, is_runtime: bool) -> Self {
		Self {
			exception_points: Vec::new(),
			except,
			is_runtime,
		}
	}

	pub fn new_not_runtime(except: Except) -> Self {
		Self::new(except, false)
	}

	pub fn new_in_runtime(except: Except) -> Self {
		Self::new(except, true)
	}

	pub fn push(&mut self, exception_point: ExceptionPoint) {
		self.exception_points.push(exception_point);
	}
}

impl ExceptionPoint {
	pub fn new(module_context: String, position: Position) -> Self {
		Self {
			module_context,
			position,
		}
	}
}

impl Except {
	fn custom(typer: ExceptType, message: String) -> Self {
		Except { typer, message }
	}

	pub fn eof(message: String) -> Self {
		Self::custom(ExceptType::Eof, message)
	}

	pub fn invalid_syntax(message: String) -> Self {
		Self::custom(ExceptType::InvalidSyntax, message)
	}

	pub fn index(message: String) -> Self {
		Self::custom(ExceptType::Index, message)
	}

	pub fn key(message: String) -> Self {
		Self::custom(ExceptType::Key, message)
	}

	pub fn keyboard_interrupt(message: String) -> Self {
		Self::custom(ExceptType::KeyboardInterrupt, message)
	}

	pub fn name(message: String) -> Self {
		Self::custom(ExceptType::Name, message)
	}

	pub fn type_(message: String) -> Self {
		Self::custom(ExceptType::Type, message)
	}

	pub fn unexpected_eof(message: String) -> Self {
		Self::custom(ExceptType::UnexpectedEOF, message)
	}
}
