// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn parse_prefix(&mut self) -> Result<Expression, Exception> {
		let mut prefix: Prefix = Prefix::Not;

		match &self.ctoken.typer {
			TokenType::BANG => prefix = Prefix::Not,
			TokenType::PLUS => prefix = Prefix::Plus,
			TokenType::MINUS => prefix = Prefix::Minus,
			_ => {},
		};

		self.next_token(true)?; // BANG or PLUS or MINUS
		Ok(Expression::Prefix(prefix, Box::new(self.parse_expression(Precedence::Lowest)?)))
	}
}
