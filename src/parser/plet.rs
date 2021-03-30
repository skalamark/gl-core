// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn parse_let(&mut self) -> Result<Statement, Exception> {
		self.next_token(true)?; // LET

		if let TokenType::IDENTIFIER(name) = self.ctoken.typer.clone() {
			self.next_token(true)?; // IDENTIFIER

			if self.ctoken.typer.is(TokenType::ASSIGN) {
				self.next_token(true)?; // ASSIGN

				let value: Expression = self.parse_expression(Precedence::Lowest)?;
				return Ok(Statement::Let(name, value));
			}

			let mut exception: Exception =
				Exception::not_runtime(Except::invalid_syntax("expected '='"));
			exception.push(ExceptionPoint::new(&self.module, self.ctoken.position.start.copy()));
			return Err(exception);
		}

		let mut exception: Exception =
			Exception::not_runtime(Except::invalid_syntax("expected identifier"));
		exception.push(ExceptionPoint::new(&self.module, self.ctoken.position.start.copy()));
		Err(exception)
	}
}
