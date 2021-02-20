// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn parse_vec(&mut self) -> Result<Literal, Exception> {
		self.next_token(true)?;
		let mut list: Vec<Expression> = Vec::new();

		while !self.ctoken.typer.is(TokenType::RightBracket) {
			let expression: Expression = self.parse_expression(Precedence::Lowest)?;
			list.push(expression);
			self.next_while_newline()?;

			match &self.ctoken.typer {
				TokenType::COMMA => self.next_token(true)?,
				TokenType::RightBracket => {}
				_ => {
					let mut exception: Exception = Exception::new(
						Except::invalid_syntax(format!("expected ',' or ']'")),
						false,
					);

					exception.push(ExceptionPoint::new(
						self.module.clone(),
						self.ctoken.position.start.copy(),
					));

					return Err(exception);
				}
			}
		}

		self.next_token(false)?;
		Ok(Literal::Vec(list))
	}
}
