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
	FLOAT(String),
	BOOLEAN(bool),
	STRING(String),

	// keywords
	LET,
	FN,
	IMPORT,

	// operators
	PLUS,             // +
	MINUS,            // -
	ASTERISK,         // *
	SLASH,            // /
	ASSIGN,           // =
	BANG,             // !
	EQUAL,            // ==
	NotEqual,         // !=
	LessThan,         // <
	LessThanEqual,    // <=
	GreaterThan,      // >
	GreaterThanEqual, // >=

	// punctuations
	DOT,          // .
	COMMA,        // ,
	SEMICOLON,    // ;
	COLON,        // :
	DCOLON,       // ::
	LeftParen,    // (
	RightParen,   // )
	LeftBracket,  // [
	RightBracket, // ]
	LeftBrace,    // {
	RightBrace,   // }
}

impl Token {
	pub fn new(typer: TokenType, position: TokenPosition) -> Self { Self { typer, position } }

	pub fn default() -> Self { Self::new(TokenType::EOF, TokenPosition::default()) }

	pub fn copy(&self) -> Self { Self::new(self.typer.clone(), self.position.copy()) }
}

impl TokenPosition {
	pub fn new(start: Position, end: Position) -> Self { Self { start, end } }

	pub fn default() -> Self { Self::new(Position::default(), Position::default()) }

	pub fn copy(&self) -> Self { Self::new(self.start.copy(), self.end.copy()) }
}

impl TokenType {
	pub fn is(&self, other: Self) -> bool { self == &other }
}
