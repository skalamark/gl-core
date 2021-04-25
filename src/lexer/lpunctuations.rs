// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::lexer::ResultLexer;
use crate::preludes::*;

impl Lexer {
	pub fn lexe_punctuations(&mut self) -> ResultLexer {
		let position_start: Position = self.position.copy();

		if self.cchar == '"' {
			return self.lexe_string();
		} else if self.operators() {
			return Ok(());
		}

		let token_type: TokenType = match self.cchar {
			c if c == '.' => TokenType::DOT,
			c if c == ',' => TokenType::COMMA,
			c if c == ';' => TokenType::SEMICOLON,
			c if c == ':' && self.nchar == ':' => {
				self.next_char();
				TokenType::DCOLON
			},
			c if c == ':' => TokenType::COLON,
			c if c == '(' => TokenType::LeftParen,
			c if c == ')' => TokenType::RightParen,
			c if c == '[' => TokenType::LeftBracket,
			c if c == ']' => TokenType::RightBracket,
			c if c == '{' => TokenType::LeftBrace,
			c if c == '}' => TokenType::RightBrace,
			_ => return self.invalid_syntax_err(),
		};

		self.next_char();
		self.make_token_and_push(token_type, position_start, self.position.copy());
		Ok(())
	}

	fn operators(&mut self) -> bool {
		let position_start: Position = self.position.copy();

		let token_type: TokenType = match self.cchar {
			c if c == '+' => TokenType::PLUS,
			c if c == '-' => TokenType::MINUS,
			c if c == '*' => TokenType::ASTERISK,
			c if c == '/' && self.nchar == '/' => {
				self.next_char();
				TokenType::CommentLine
			},
			c if c == '/' => TokenType::SLASH,
			c if c == '=' && self.nchar == '=' => {
				self.next_char();
				TokenType::EQUAL
			},
			c if c == '=' => TokenType::ASSIGN,
			c if c == '!' && self.nchar == '=' => {
				self.next_char();
				TokenType::NotEqual
			},
			c if c == '!' => TokenType::BANG,
			c if c == '<' && self.nchar == '=' => {
				self.next_char();
				TokenType::LessThanEqual
			},
			c if c == '<' => TokenType::LessThan,
			c if c == '>' && self.nchar == '=' => {
				self.next_char();
				TokenType::GreaterThanEqual
			},
			c if c == '>' => TokenType::GreaterThan,
			_ => return false,
		};

		self.next_char();
		self.make_token_and_push(token_type, position_start, self.position.copy());
		true
	}
}
