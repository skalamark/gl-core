// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn parse_call(&mut self, left: Expression) -> Result<Expression, Exception> {
		self.next_token(true)?; // LeftParen
		let mut arguments: Vec<Expression> = Vec::new();

		while !self.ctoken.typer.is(TokenType::RightParen) {
			arguments.push(self.parse_expression(Precedence::Comma)?);
			self.next_while_newline()?;

			match &self.ctoken.typer {
				TokenType::COMMA => self.next_token(true)?,
				TokenType::RightParen => {},
				_ => {
					let mut exception: Exception =
						Exception::not_runtime(Except::invalid_syntax("expected ',' or ')'"));
					exception
						.push(ExceptionPoint::new(&self.module, self.ctoken.position.start.copy()));
					return Err(exception);
				},
			}
		}

		self.next_token(false)?; // RightParen
		Ok(Expression::Call { function: Box::new(left), arguments })
	}
}
