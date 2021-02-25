// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn parse_let(&mut self) -> Result<Statement, Exception> {
		self.next_token(true)?;

		let name: String = match self.ctoken.typer.clone() {
			TokenType::IDENTIFIER(name) => name,
			_ => {
				let mut exception: Exception = Exception::new(
					Except::invalid_syntax(format!("expected identifier'")),
					false,
				);

				exception.push(ExceptionPoint::new(
					self.module.clone(),
					self.ctoken.position.start.copy(),
				));

				return Err(exception);
			}
		};

		self.next_token(true)?;

		if !self.ctoken.typer.is(TokenType::ASSIGN) {
			let mut exception: Exception =
				Exception::new(Except::invalid_syntax(format!("expected '='")), false);

			exception.push(ExceptionPoint::new(
				self.module.clone(),
				self.ctoken.position.start.copy(),
			));

			return Err(exception);
		}

		self.next_token(true)?;
		let value: Expression = self.parse_expression(Precedence::Lowest)?;
		Ok(Statement::Let(name, value))
	}
}
