// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, Exception> {
		// prefix
		if self.ctoken.typer.is(TokenType::EOF) {
			let mut exception: Exception = Exception::new(
				Except::unexpected_eof(format!("unexpected EOF while parsing")),
				false,
			);

			exception.push(ExceptionPoint::new(
				self.module.clone(),
				self.ctoken.position.start.copy(),
			));

			return Err(exception);
		}

		let mut left: Expression = match self.ctoken.typer.clone() {
			TokenType::IDENTIFIER(identifier) => {
				self.next_token(false)?;
				match &self.ctoken.typer {
					TokenType::DOT => {
						self.next_token(true)?;
						let attribute_expression: Expression =
							self.parse_expression(Precedence::Lowest)?;
						Expression::Attribute(
							Box::new(Expression::Identifier(identifier)),
							Box::new(attribute_expression),
						)
					}
					_ => Expression::Identifier(identifier),
				}
			}
			TokenType::NULL => {
				self.next_token(false)?;
				Expression::Literal(Literal::Null)
			}
			TokenType::INTEGER(integer_literal) => {
				self.next_token(false)?;
				Expression::Literal(self.parse_integer(integer_literal)?)
			}
			TokenType::FLOAT(float_literal) => {
				self.next_token(false)?;
				Expression::Literal(self.parse_float(float_literal)?)
			}
			TokenType::BOOLEAN(boolean_literal) => {
				self.next_token(false)?;
				Expression::Literal(Literal::Boolean(boolean_literal))
			}
			TokenType::STRING(string_literal) => {
				self.next_token(false)?;
				Expression::Literal(Literal::String(string_literal))
			}
			TokenType::LeftBracket => Expression::Literal(self.parse_vec()?),
			TokenType::LeftBrace => Expression::Literal(self.parse_hashmap()?),
			TokenType::BANG | TokenType::PLUS | TokenType::MINUS => self.parse_prefix()?,
			TokenType::FN => self.parse_function_anonymous()?,
			TokenType::IF => self.parse_if()?,
			_ => {
				let mut exception: Exception =
					Exception::new(Except::invalid_syntax(format!("invalid token")), false);

				exception.push(ExceptionPoint::new(
					self.module.clone(),
					self.ctoken.position.start.copy(),
				));

				return Err(exception);
			}
		};

		// infix
		while precedence < Precedence::from_token(&self.ctoken) {
			match &self.ctoken.typer {
				TokenType::PLUS
				| TokenType::MINUS
				| TokenType::ASTERISK
				| TokenType::SLASH
				| TokenType::EQUAL
				| TokenType::NotEqual
				| TokenType::LessThan
				| TokenType::LessThanEqual
				| TokenType::GreaterThan
				| TokenType::GreaterThanEqual => left = self.parse_infix(left)?,
				TokenType::LeftParen => left = self.parse_call(left)?,
				TokenType::LeftBracket => left = self.parse_index(left)?,
				_ => {}
			}
		}

		Ok(left)
	}
}
