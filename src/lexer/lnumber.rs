// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::lexer::ResultLexer;
use crate::preludes::*;

impl Lexer {
	pub fn number(&mut self) -> ResultLexer {
		let position_start: Position = self.position.copy();
		let mut number_literal: String = String::new();
		let mut dot: bool = false;

		while !self.is_eof_char() && (self.cchar.is_digit(10) || self.cchar == '.') {
			if self.cchar == '.' {
				if !dot {
					dot = true
				} else {
					let mut exception: Exception = Exception::new_not_runtime(
						Except::invalid_syntax(format!("invalid character '{}'", &self.cchar)),
					);
					exception.push(ExceptionPoint::new(
						self.module.clone(),
						self.position.copy(),
					));
					return Err(exception);
				}
			}

			number_literal.push(self.cchar);
			self.next_char()
		}

		if !dot {
			self.push_token_in_cache(Token::new(
				TokenType::INTEGER(number_literal),
				TokenPosition::new(position_start, self.position.copy()),
			));
		} else {
			self.push_token_in_cache(Token::new(
				TokenType::FLOAT(number_literal),
				TokenPosition::new(position_start, self.position.copy()),
			));
		}

		Ok(())
	}
}
