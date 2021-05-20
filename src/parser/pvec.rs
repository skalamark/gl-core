// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn parse_vec(&mut self) -> Result<Literal, Exception> {
		self.next_token(true)?; // LeftBracket
		let mut values: Vec<Expression> = Vec::new();

		while !self.ctoken.typer.is(TokenType::RightBracket) {
			values.push(self.parse_expression(Precedence::Comma)?);
			self.next_while_newline()?;

			match &self.ctoken.typer {
				TokenType::COMMA => self.next_token(true)?, // COMMA
				TokenType::RightBracket => {},
				_ => {
					let mut exception: Exception =
						Exception::not_runtime(Except::invalid_syntax("expected ',' or ']'"));
					exception
						.push(ExceptionPoint::new(&self.module, self.ctoken.position.start.copy()));
					return Err(exception);
				},
			}
		}

		self.next_token(false)?; // RightBracket
		Ok(Literal::Vec(values))
	}
}
