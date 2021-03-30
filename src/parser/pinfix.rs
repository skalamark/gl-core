// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn parse_infix(&mut self, left: Expression) -> Result<Expression, Exception> {
		let mut infix: Infix = Infix::Plus;

		match &self.ctoken.typer {
			TokenType::PLUS => infix = Infix::Plus,
			TokenType::MINUS => infix = Infix::Minus,
			TokenType::ASTERISK => infix = Infix::Multiply,
			TokenType::SLASH => infix = Infix::Divide,
			TokenType::EQUAL => infix = Infix::Equal,
			TokenType::NotEqual => infix = Infix::NotEqual,
			TokenType::LessThan => infix = Infix::LessThan,
			TokenType::LessThanEqual => infix = Infix::LessThanEqual,
			TokenType::GreaterThan => infix = Infix::GreaterThan,
			TokenType::GreaterThanEqual => infix = Infix::GreaterThanEqual,
			_ => {},
		};

		let precedence: Precedence = Precedence::from_token(&self.ctoken);
		self.next_token(true)?;
		Ok(Expression::Infix(infix, Box::new(left), Box::new(self.parse_expression(precedence)?)))
	}
}
