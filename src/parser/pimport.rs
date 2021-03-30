// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn parse_import(&mut self) -> Result<Statement, Exception> {
		self.next_token(true)?; // IMPORT

		if let TokenType::STRING(name) = self.ctoken.typer.clone() {
			self.next_token(true)?; // STRING
			return Ok(Statement::Import(name));
		}

		let mut exception: Exception =
			Exception::not_runtime(Except::invalid_syntax("expected string"));
		exception.push(ExceptionPoint::new(&self.module, self.ctoken.position.start.copy()));
		Err(exception)
	}
}
