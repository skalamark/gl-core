// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::error::{AnyError, Error};
use crate::position::Position;
use crate::state::ProgramState;
use crate::token::Token;

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
			self.cchar = self.chars.remove(0);
		} else {
			self.cchar = '\0';
		}
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
		match &self.cchar {
			c if c.is_whitespace() => match self.lexe_whitespace(tokens, module, program) {
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
			if self.cchar == '\0' {
				break;
			}

			match self.lexe_token(&mut tokens, module, program) {
				Ok(_) => {}
				Err(exception) => return Err(exception),
			}
		}

		Ok(tokens)
	}
}
