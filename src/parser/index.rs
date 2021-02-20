// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn parse_index(&mut self, left: Expression) -> Result<Expression, Exception> {
		self.next_token(true)?;
		let index: Expression = self.parse_expression(Precedence::Lowest)?;

		if !self.ctoken.typer.is(TokenType::RightBracket) {
			let mut exception: Exception =
				Exception::new(Except::invalid_syntax(format!("expected ']'")), false);

			exception.push(ExceptionPoint::new(
				self.module.clone(),
				self.ctoken.position.start.copy(),
			));

			return Err(exception);
		}

		self.next_token(false)?;
		Ok(Expression::Index(Box::new(left), Box::new(index)))
	}
}
