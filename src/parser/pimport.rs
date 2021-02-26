// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn parse_import(&mut self) -> Result<Statement, Exception> {
		self.next_token(true)?;

		let name: String = match self.ctoken.typer.clone() {
			TokenType::STRING(name) => name,
			_ => {
				let mut exception: Exception =
					Exception::new(Except::invalid_syntax(format!("expected string")), false);

				exception.push(ExceptionPoint::new(
					self.module.clone(),
					self.ctoken.position.start.copy(),
				));

				return Err(exception);
			}
		};

		self.next_token(true)?;
		Ok(Statement::Import(name))
	}
}
