// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::lexer::ResultLexer;
use crate::preludes::*;

impl Lexer {
	pub fn lexe_whitespace(&mut self) -> ResultLexer {
		let mut position_start: Position = self.position.copy();

		while !self.is_eof_char() && self.cchar.is_whitespace() {
			if self.cchar == '\n' {
				self.position.column += 1;
				self.make_token_and_push(
					TokenType::NEWLINE,
					position_start.copy(),
					self.position.copy(),
				);
				self.next_char();
				self.position.column = 0;
				self.position.line += 1;
				continue;
			}

			self.next_char();
			position_start = self.position.copy();
		}

		Ok(())
	}
}
