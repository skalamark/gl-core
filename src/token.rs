// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::position::Position;

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
	pub typer: TokenType,
	pub position: TokenPosition,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TokenPosition {
	pub start: Position,
	pub end: Position,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
	EOF,     // \0
	NEWLINE, // \n

	// identifier and literals
	IDENTIFIER(String),
	NULL,
	INTEGER(String),
	BOOLEAN(bool),
	STRING(String),

	// keywords
	LET,

	// operators
	PLUS,             // +
	MINUS,            // -
	MULTIPLY,         // *
	DIVIDE,           // /
	EQUAL,            // ==
	NotEqual,         // !=
	ASSIGN,           // =
	NOT,              // !
	LessThan,         // <
	GreaterThan,      // >
	LessThanEqual,    // <=
	GreaterThanEqual, // >=

	// punctuations
	DOT,       // .
	COMMA,     // ,
	SEMICOLON, // ;
	COLON,     // :
	LParen,    // (
	RParen,    // )
	LBracket,  // [
	RBracket,  // ]
	LBrace,    // {
	RBrace,    // }
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
