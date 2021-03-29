// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::lexer::ResultLexer;
use crate::preludes::*;

impl Lexer {
	pub fn lexe_string(&mut self) -> ResultLexer {
		let position_start: Position = self.position.copy();
		let mut string_literal: String = String::new();
		let mut escape: bool = false;
		self.next_char();

		while !self.is_eof_char() && (self.cchar != '"' || escape) {
			if escape {
				match self.cchar {
					'\\' => string_literal.push('\\'),
					'"' => string_literal.push('\"'),
					'n' => string_literal.push('\n'),
					'r' => string_literal.push('\r'),
					't' => string_literal.push('\t'),
					c => {
						let mut exception: Exception = Exception::not_runtime(
							Except::invalid_syntax(format!("unknown character escape: `{}`", c)),
						);
						exception.push(ExceptionPoint::new(&self.module, self.position.copy()));
						return Err(exception);
					},
				}
				escape = false
			} else {
				if self.cchar == '\\' {
					escape = true;
				} else {
					string_literal.push(self.cchar);
				}
			}
			self.next_char()
		}

		if self.cchar != '"' {
			let mut exception: Exception =
				Exception::not_runtime(Except::unexpected_eof("unterminated double quote string"));
			exception.push(ExceptionPoint::new(&self.module, self.position.copy()));
			return Err(exception);
		}

		self.next_char();
		self.make_token_and_push(
			TokenType::STRING(string_literal),
			position_start,
			self.position.copy(),
		);

		Ok(())
	}
}
