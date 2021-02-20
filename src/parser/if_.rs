// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn parse_if(&mut self) -> Result<Expression, Exception> {
		self.next_token(true)?;

		let condition: Expression = self.parse_expression(Precedence::Lowest)?;
		let consequence: Block = self.parse_block()?;
		let alternative: Option<Block> = if self.ctoken.typer.is(TokenType::ELSE) {
			self.next_token(true)?;
			Some(self.parse_block()?)
		} else {
			None
		};

		Ok(Expression::If {
			condition: Box::new(condition),
			consequence,
			alternative,
		})
	}
}
