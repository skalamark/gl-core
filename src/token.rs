// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::position::Position;

pub struct Token {
	pub typer: TokenType,
	pub position: TokenPosition,
}

pub struct TokenPosition {
	pub start: Position,
	pub end: Position,
}

pub enum TokenType {
	EOF, // \0
}

impl Token {
	pub fn new(typer: TokenType, position: TokenPosition) -> Self {
		Self { typer, position }
	}
}

impl TokenPosition {
	pub fn new(start: Position, end: Position) -> Self {
		Self { start, end }
	}

	pub fn default() -> Self {
		Self::new(Position::default(), Position::default())
	}

	pub fn copy(&self) -> Self {
		Self::new(self.start.copy(), self.end.copy())
	}
}

impl TokenType {
	pub fn is_eof(&self) -> bool {
		match self {
			TokenType::EOF => true,
			_ => false,
		}
	}
}
