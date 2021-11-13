// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn parse_property(&mut self, left: Expression) -> Result<Expression, Exception> {
		self.next_token(true)?; // DCOLON

		let property: Expression = if self.ctoken.typer.is(TokenType::LeftBrace) {
			self.next_token(true)?;
			let mut propertys: Vec<Expression> = Vec::new();

			while !self.ctoken.typer.is(TokenType::RightBrace) {
				propertys.push(self.parse_expression(Precedence::Comma)?);
				self.next_while_newline()?;

				match &self.ctoken.typer {
					TokenType::COMMA => self.next_token(true)?,
					TokenType::RightBrace => {},
					_ => {
						let mut exception: Exception =
							Exception::not_runtime(Except::invalid_syntax("expected ',' or '}'"));
						exception.push(ExceptionPoint::new(
							&self.module,
							self.ctoken.position.start.copy(),
						));
						return Err(exception);
					},
				}
			}

			self.next_token(false)?; // RightBrace
			Expression::Literal(Literal::Vec(propertys))
		} else {
			if let TokenType::IDENTIFIER(_) = self.ctoken.typer.clone() {
				self.parse_expression(Precedence::Lowest)?
			} else {
				let mut exception: Exception =
					Exception::not_runtime(Except::invalid_syntax("expected identifier"));
				exception
					.push(ExceptionPoint::new(&self.module, self.ctoken.position.start.copy()));
				return Err(exception);
			}
		};

		Ok(Expression::Property(Box::new(left), Box::new(property)))
	}
}
