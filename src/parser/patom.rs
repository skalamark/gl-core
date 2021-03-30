// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn parse_atom(&mut self) -> Result<Expression, Exception> {
		if self.ctoken.typer.is(TokenType::EOF) {
			let mut exception: Exception =
				Exception::not_runtime(Except::unexpected_eof("unexpected EOF while parsing"));
			exception.push(ExceptionPoint::new(&self.module, self.ctoken.position.start.copy()));
			return Err(exception);
		}

		// prefix
		let left: Expression = match self.ctoken.typer.clone() {
			TokenType::IDENTIFIER(identifier) => {
				self.next_token(false)?; // IDENTIFIER
				Expression::Identifier(identifier)
			},
			TokenType::NULL => {
				self.next_token(false)?; // NULL
				Expression::Literal(Literal::Null)
			},
			TokenType::INTEGER(integer_literal) => {
				self.next_token(false)?; // INTEGER
				Expression::Literal(self.parse_integer(integer_literal)?)
			},
			TokenType::FLOAT(float_literal) => {
				self.next_token(false)?; // FLOAT
				Expression::Literal(self.parse_float(float_literal)?)
			},
			TokenType::BOOLEAN(boolean_literal) => {
				self.next_token(false)?; // BOOLEAN
				Expression::Literal(Literal::Boolean(boolean_literal))
			},
			TokenType::STRING(string_literal) => {
				self.next_token(false)?; // STRING
				Expression::Literal(Literal::String(string_literal))
			},
			TokenType::BANG | TokenType::PLUS | TokenType::MINUS => self.parse_prefix()?,
			TokenType::FN => self.parse_function_anonymous()?,
			TokenType::LeftParen => unimplemented!(),
			TokenType::LeftBracket => Expression::Literal(self.parse_vec()?),
			TokenType::LeftBrace => Expression::Literal(self.parse_hashmap()?),
			_ => {
				let mut exception: Exception =
					Exception::not_runtime(Except::invalid_syntax("Unexpected token"));
				exception
					.push(ExceptionPoint::new(&self.module, self.ctoken.position.start.copy()));
				return Err(exception);
			},
		};

		Ok(left)
	}
}
