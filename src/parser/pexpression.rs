// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn parse_expression(&mut self, precedence: Precedence) -> Result<Expression, Exception> {
		// prefix
		let mut left: Expression = self.parse_atom()?;

		// infix
		while precedence < Precedence::from_token(&self.ctoken) {
			left = match &self.ctoken.typer {
				TokenType::COMMA => self.parse_tuple(Some(left))?,
				TokenType::PLUS
				| TokenType::MINUS
				| TokenType::ASTERISK
				| TokenType::SLASH
				| TokenType::EQUAL
				| TokenType::NotEqual
				| TokenType::LessThan
				| TokenType::LessThanEqual
				| TokenType::GreaterThan
				| TokenType::GreaterThanEqual => self.parse_infix(left)?,
				TokenType::LeftParen => self.parse_call(left)?,
				TokenType::LeftBracket => self.parse_index(left)?,
				TokenType::DCOLON => unimplemented!(),
				_ => left,
			};
		}

		Ok(left)
	}
}
