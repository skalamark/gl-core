// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::error::{AnyError, Error};
use crate::position::Position;
use crate::state::ProgramState;
use crate::token::Token;
use crate::token::TokenPosition;

pub struct Lexer {
	cchar: char,
	chars: Vec<char>,
	position: Position,
}

impl Lexer {
	pub fn new() -> Self {
		Self {
			cchar: '\0',
			chars: Vec::new(),
			position: Position::default(),
		}
	}

	fn next(&mut self) {
		if self.chars.len() > 0 {
			self.position.column += 1;
			self.cchar = self.chars.remove(0);
		} else {
			self.cchar = '\0';
		}
	}

	fn lexe_digits(
		&mut self, tokens: &mut Vec<Token>, module: &String, program: &mut ProgramState,
	) -> Result<(), AnyError> {
		let mut digits_literal: String = String::new();
		let position_start: Position = self.position.copy();

		while self.cchar != '\0' && self.cchar.is_digit(10) {
			digits_literal.push(self.cchar);
			self.next()
		}

		tokens.push(Token::new(
			crate::token::TokenType::INTEGER(digits_literal),
			TokenPosition::new(position_start, self.position.copy()),
		));

		Ok(())
	}

	fn lexe_whitespace(
		&mut self, tokens: &mut Vec<Token>, module: &String, program: &mut ProgramState,
	) -> Result<(), AnyError> {
		while self.cchar != '\0' && self.cchar.is_whitespace() {
			self.next()
		}

		Ok(())
	}

	fn lexe_token(
		&mut self, tokens: &mut Vec<Token>, module: &String, program: &mut ProgramState,
	) -> Result<(), AnyError> {
		if self.chars.len() == 0 {
			self.next();
			tokens.push(Token::new(
				crate::token::TokenType::EOF,
				TokenPosition::new(self.position.copy(), self.position.copy()),
			));
			return Ok(());
		}

		match self.cchar {
			c if c.is_whitespace() => match self.lexe_whitespace(tokens, module, program) {
				Ok(_) => {}
				Err(exception) => return Err(exception),
			},
			c if c.is_digit(10) => match self.lexe_digits(tokens, module, program) {
				Ok(_) => {}
				Err(exception) => return Err(exception),
			},
			_ => return Err(Error::invalid_syntax(format!("invalid token"))),
		}

		Ok(())
	}

	pub fn run(
		&mut self, source: String, module: &String, program: &mut ProgramState,
	) -> Result<Vec<Token>, AnyError> {
		self.chars = source.chars().collect();
		let mut tokens: Vec<Token> = Vec::new();
		self.next();

		loop {
			if let Some(last_token) = tokens.last() {
				if last_token.typer.is_eof() {
					break;
				}
			}

			match self.lexe_token(&mut tokens, module, program) {
				Ok(_) => {}
				Err(exception) => return Err(exception),
			}
		}

		Ok(tokens)
	}
}
