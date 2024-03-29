// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::lexer::ResultLexer;
use crate::preludes::*;

impl Lexer {
	pub fn lexe_identifier_keyword(&mut self) -> ResultLexer {
		let position_start: Position = self.position.copy();
		let mut identifier_literal: String = String::new();

		while !self.is_eof_char()
			&& (self.cchar.is_alphabetic() || self.cchar.is_digit(10) || self.cchar == '_')
		{
			identifier_literal.push(self.cchar);
			self.next_char()
		}

		let token_type: TokenType = match identifier_literal {
			i if i == "null" => TokenType::NULL,
			i if i == "true" => TokenType::BOOLEAN(true),
			i if i == "false" => TokenType::BOOLEAN(false),
			i if i == "let" => TokenType::LET,
			i if i == "fn" => TokenType::FN,
			i if i == "import" => TokenType::IMPORT,
			identifier => TokenType::IDENTIFIER(identifier),
		};

		self.make_token_and_push(token_type, position_start, self.position.copy());
		Ok(())
	}
}
