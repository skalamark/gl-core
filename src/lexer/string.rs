// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Lexer {
	pub fn string(&mut self) -> ResultLexer {
		let position_start: Position = self.position.copy();
		let mut string_literal: String = String::new();
		self.next_char();

		while !self.is_eof_char() && self.cchar != '"' {
			string_literal.push(self.cchar);
			self.next_char()
		}

		if self.cchar != '"' {
			let mut exception: Exception = Exception::new_not_runtime(Except::unexpected_eof(
				format!("EOL while scanning string literal"),
			));
			exception.push(ExceptionPoint::new(
				self.module.clone(),
				self.position.copy(),
			));
			return Err(exception);
		}
		self.next_char();

		self.push_token_in_cache(Token::new(
			TokenType::STRING(string_literal),
			TokenPosition::new(position_start, self.position.copy()),
		));

		Ok(())
	}
}
