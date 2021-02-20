// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Lexer {
	pub fn punctuations(&mut self) -> ResultLexer {
		let position_start: Position = self.position.copy();

		if self.cchar == '"' {
			return self.string();
		} else if self.operators() {
			return Ok(());
		}

		let token_type: TokenType = match self.cchar {
			c if c == '.' => {
				self.next_char();
				TokenType::DOT
			}
			c if c == ',' => {
				self.next_char();
				TokenType::COMMA
			}
			c if c == ';' => {
				self.next_char();
				TokenType::SEMICOLON
			}
			c if c == ':' => {
				self.next_char();
				TokenType::COLON
			}
			c if c == '(' => {
				self.next_char();
				TokenType::LeftParen
			}
			c if c == ')' => {
				self.next_char();
				TokenType::RightParen
			}
			c if c == '[' => {
				self.next_char();
				TokenType::LeftBracket
			}
			c if c == ']' => {
				self.next_char();
				TokenType::RightBracket
			}
			c if c == '{' => {
				self.next_char();
				TokenType::LeftBrace
			}
			c if c == '}' => {
				self.next_char();
				TokenType::RightBrace
			}
			_ => {
				let mut exception: Exception = Exception::new_not_runtime(Except::invalid_syntax(
					format!("invalid character '{}'", &self.cchar),
				));
				exception.push(ExceptionPoint::new(
					self.module.clone(),
					self.position.copy(),
				));
				return Err(exception);
			}
		};

		self.push_token_in_cache(Token::new(
			token_type,
			TokenPosition::new(position_start, self.position.copy()),
		));

		Ok(())
	}

	fn operators(&mut self) -> bool {
		let position_start: Position = self.position.copy();

		let token_type: TokenType = match self.cchar {
			c if c == '+' => {
				self.next_char();
				TokenType::PLUS
			}
			c if c == '-' => {
				self.next_char();
				TokenType::MINUS
			}
			c if c == '*' => {
				self.next_char();
				TokenType::ASTERISK
			}
			c if c == '/' => {
				self.next_char();
				TokenType::SLASH
			}
			c if c == '=' => {
				self.next_char();
				if self.cchar == '=' {
					self.next_char();
					TokenType::EQUAL
				} else {
					TokenType::ASSIGN
				}
			}
			c if c == '!' => {
				self.next_char();
				if self.cchar == '=' {
					self.next_char();
					TokenType::NotEqual
				} else {
					TokenType::BANG
				}
			}
			c if c == '>' => {
				self.next_char();
				if self.cchar == '=' {
					self.next_char();
					TokenType::GreaterThanEqual
				} else {
					TokenType::GreaterThan
				}
			}
			c if c == '<' => {
				self.next_char();
				if self.cchar == '=' {
					self.next_char();
					TokenType::LessThanEqual
				} else {
					TokenType::LessThan
				}
			}
			_ => return false,
		};

		self.push_token_in_cache(Token::new(
			token_type,
			TokenPosition::new(position_start, self.position.copy()),
		));

		true
	}
}
