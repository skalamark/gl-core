// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn parse_hashmap(&mut self) -> Result<Literal, Exception> {
		self.next_token(true)?;
		let mut list: Vec<(Expression, Expression)> = Vec::new();

		while !self.ctoken.typer.is(TokenType::RightBrace) {
			let key: Expression = self.parse_expression(Precedence::Lowest)?;

			if !self.ctoken.typer.is(TokenType::COLON) {
				let mut exception: Exception =
					Exception::new(Except::invalid_syntax(format!("expected ':'")), false);

				exception.push(ExceptionPoint::new(
					self.module.clone(),
					self.ctoken.position.start.copy(),
				));

				return Err(exception);
			}
			self.next_token(true)?;

			let value: Expression = self.parse_expression(Precedence::Lowest)?;
			list.push((key, value));
			self.next_while_newline()?;

			match &self.ctoken.typer {
				TokenType::COMMA => self.next_token(true)?,
				TokenType::RightBrace => {}
				_ => {
					let mut exception: Exception = Exception::new(
						Except::invalid_syntax(format!("expected ',' or '}}'")),
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
		Ok(Literal::HashMap(list))
	}
}
