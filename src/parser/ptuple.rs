// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn parse_tuple(
		&mut self, first_value: Option<Expression>,
	) -> Result<Expression, Exception> {
		self.next_token(true)?; // LeftParen or COMMA
		let mut values: Vec<Expression> = Vec::new();
		let mut exists_first_value: bool = false; // Is LeftParen

		if let Some(first) = first_value {
			exists_first_value = true; // Is COMMA
			values.push(first);
		}

		while (exists_first_value)
			|| (!exists_first_value && !self.ctoken.typer.is(TokenType::RightParen))
		{
			values.push(self.parse_expression(Precedence::Comma)?);
			if !exists_first_value {
				self.next_while_newline()?;
			}

			match &self.ctoken.typer {
				TokenType::COMMA if exists_first_value => self.next_token(false)?, // COMMA
				TokenType::COMMA if !exists_first_value => self.next_token(true)?, // COMMA
				TokenType::RightParen if !exists_first_value => {},
				_ if !exists_first_value => {
					let mut exception: Exception =
						Exception::not_runtime(Except::invalid_syntax("expected ',' or ')'"));
					exception
						.push(ExceptionPoint::new(&self.module, self.ctoken.position.start.copy()));
					return Err(exception);
				},
				_ => break,
			}
		}

		if !exists_first_value {
			self.next_token(false)?; // RightParen
		}

		if values.len() == 1 {
			Ok(values.remove(0))
		} else {
			Ok(Expression::Literal(Literal::Tuple(values)))
		}
	}
}
