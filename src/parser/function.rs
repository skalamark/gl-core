// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn is_fn_statement_anonymous(&self) -> Result<&str, Exception> {
		match &self.ntoken.typer {
			TokenType::IDENTIFIER(_) => return Ok("statement"),
			TokenType::LeftParen => return Ok("expression"),
			_ => {}
		}

		let mut exception: Exception = Exception::new(
			Except::invalid_syntax(format!("expected identifier or '('")),
			false,
		);

		exception.push(ExceptionPoint::new(
			self.module.clone(),
			self.ntoken.position.start.copy(),
		));

		return Err(exception);
	}

	pub fn parse_function(&mut self) -> Result<Statement, Exception> {
		self.next_token(true)?;

		let name: String = match self.ctoken.typer.clone() {
			TokenType::IDENTIFIER(name) => name,
			_ => {
				let mut exception: Exception = Exception::new(
					Except::invalid_syntax(format!("expected identifier'")),
					false,
				);

				exception.push(ExceptionPoint::new(
					self.module.clone(),
					self.ctoken.position.start.copy(),
				));

				return Err(exception);
			}
		};

		self.next_token(true)?;
		let mut params: Vec<String> = Vec::new();

		if !self.ctoken.typer.is(TokenType::LeftParen) {
			let mut exception: Exception =
				Exception::new(Except::invalid_syntax(format!("expected '('")), false);

			exception.push(ExceptionPoint::new(
				self.module.clone(),
				self.ctoken.position.start.copy(),
			));

			return Err(exception);
		}

		self.next_token(true)?;

		while !self.ctoken.typer.is(TokenType::RightParen) {
			if let TokenType::IDENTIFIER(argument) = &self.ctoken.typer {
				params.push(argument.clone());
			}
			self.next_token(true)?;

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
		self.next_token(true)?;

		let body: Block = self.parse_block()?;
		Ok(Statement::Fn { name, params, body })
	}

	pub fn parse_function_anonymous(&mut self) -> Result<Expression, Exception> {
		self.next_token(true)?;
		let mut params: Vec<String> = Vec::new();

		if !self.ctoken.typer.is(TokenType::LeftParen) {
			let mut exception: Exception =
				Exception::new(Except::invalid_syntax(format!("expected '('")), false);

			exception.push(ExceptionPoint::new(
				self.module.clone(),
				self.ctoken.position.start.copy(),
			));

			return Err(exception);
		}

		self.next_token(true)?;

		while self.ctoken.typer != TokenType::RightParen {
			if let TokenType::IDENTIFIER(argument) = &self.ctoken.typer {
				params.push(argument.clone());
			}
			self.next_token(true)?;

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
		self.next_token(true)?;

		let body: Block = self.parse_block()?;
		Ok(Expression::Fn { params, body })
	}
}
