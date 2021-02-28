// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn parse_call(&mut self, function: Expression) -> Result<Expression, Exception> {
		self.next_token(true)?; // LeftParen
		let mut arguments: Vec<Expression> = Vec::new();

		while !self.ctoken.typer.is(TokenType::RightParen) {
			let expression: Expression = self.parse_expression(Precedence::Lowest)?;
			arguments.push(expression);
			self.next_while_newline()?;

			match &self.ctoken.typer {
				TokenType::COMMA => self.next_token(true)?,
				TokenType::RightParen => {}
				_ => {
					let mut exception: Exception = Exception::new(
						Except::invalid_syntax(format!("expected ',' or ')'")),
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

		self.next_token(false)?; // RightParen
		Ok(Expression::Call {
			function: Box::new(function),
			arguments,
		})
	}
}
