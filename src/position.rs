// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
	pub column: usize,
	pub line: usize,
}

impl Position {
	pub fn new(column: usize, line: usize) -> Self {
		Self { column, line }
	}

	pub fn default() -> Self {
		Self::new(0, 0)
	}

	pub fn copy(&self) -> Self {
		Self::new(self.column, self.line)
	}
}
