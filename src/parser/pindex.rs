// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn parse_index(&mut self, left: Expression) -> Result<Expression, Exception> {
		self.next_token(true)?; // LeftBracket
		let index: Expression = self.parse_expression(Precedence::Lowest)?;
		self.next_while_newline()?;

		if !self.ctoken.typer.is(TokenType::RightBracket) {
			let mut exception: Exception =
				Exception::not_runtime(Except::invalid_syntax("expected ']'"));
			exception.push(ExceptionPoint::new(&self.module, self.ctoken.position.start.copy()));
			return Err(exception);
		}

		self.next_token(false)?; // RightBracket
		Ok(Expression::Index(Box::new(left), Box::new(index)))
	}
}
