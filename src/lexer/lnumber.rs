// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::lexer::ResultLexer;
use crate::preludes::*;

impl Lexer {
	pub fn lexe_number(&mut self) -> ResultLexer {
		let position_start: Position = self.position.copy();
		let mut number_literal: String = String::new();
		let mut dot: bool = false;

		while !self.is_eof_char() && (self.cchar.is_digit(10) || self.cchar == '.') {
			if self.cchar == '.' {
				if dot {
					return self.invalid_syntax_err();
				}
				dot = true;
			}

			number_literal.push(self.cchar);
			self.next_char();
		}

		self.make_token_and_push(
			if !dot {
				TokenType::INTEGER(number_literal)
			} else {
				TokenType::FLOAT(number_literal)
			},
			position_start,
			self.position.copy(),
		);

		Ok(())
	}
}
