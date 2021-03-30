// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn is_fn_statement_anonymous(&mut self) -> Result<&str, Exception> {
		if let TokenType::IDENTIFIER(_) = &self.ntoken.typer {
			return Ok("statement");
		} else if self.ntoken.typer.is(TokenType::LeftParen) {
			return Ok("anonymous");
		}

		let mut tc: Vec<Token> = Vec::new();
		loop {
			let tl = self.next_token_from_lexer()?;

			if let TokenType::IDENTIFIER(_) = &tl.typer {
				for t in tc {
					self.tokens_cache.push(t)
				}
				self.tokens_cache.push(tl);
				return Ok("statement");
			} else if tl.typer.is(TokenType::LeftParen) {
				for t in tc {
					self.tokens_cache.push(t)
				}
				self.tokens_cache.push(tl);
				return Ok("anonymous");
			} else if !tl.typer.is(TokenType::NEWLINE) {
				for t in tc {
					self.tokens_cache.push(t)
				}
				self.tokens_cache.push(tl);
				break;
			}

			tc.push(tl);
		}

		let mut exception: Exception =
			Exception::not_runtime(Except::invalid_syntax("expected identifier or '('"));
		exception.push(ExceptionPoint::new(&self.module, self.ntoken.position.start.copy()));
		Err(exception)
	}

	pub fn parse_function(&mut self) -> Result<Statement, Exception> {
		self.next_token(true)?; // FN

		let name: String = match self.ctoken.typer.clone() {
			TokenType::IDENTIFIER(name) => name,
			_ => {
				let mut exception: Exception =
					Exception::not_runtime(Except::invalid_syntax("expected identifier'"));
				exception
					.push(ExceptionPoint::new(&self.module, self.ctoken.position.start.copy()));
				return Err(exception);
			},
		};

		self.next_token(true)?; // IDENTIFIER
		let mut params: Vec<String> = Vec::new();

		if !self.ctoken.typer.is(TokenType::LeftParen) {
			let mut exception: Exception =
				Exception::not_runtime(Except::invalid_syntax("expected '('"));
			exception.push(ExceptionPoint::new(&self.module, self.ctoken.position.start.copy()));
			return Err(exception);
		}
		self.next_token(true)?; // LeftParen

		while !self.ctoken.typer.is(TokenType::RightParen) {
			if let TokenType::IDENTIFIER(argument) = self.ctoken.typer.clone() {
				params.push(argument);
				self.next_token(true)?; // IDENTIFIER
			}

			match &self.ctoken.typer {
				TokenType::COMMA => {
					if params.len() > 0 {
						self.next_token(true)?; // COMMA
					} else {
						let mut exception: Exception =
							Exception::not_runtime(Except::invalid_syntax("invalid syntax"));
						exception.push(ExceptionPoint::new(
							&self.module,
							self.ctoken.position.start.copy(),
						));
						return Err(exception);
					}
				},
				TokenType::RightParen => {},
				_ => {
					let mut exception: Exception =
						Exception::not_runtime(Except::invalid_syntax("expected ',' or ')'"));
					exception
						.push(ExceptionPoint::new(&self.module, self.ctoken.position.start.copy()));
					return Err(exception);
				},
			}
		}

		self.next_token(true)?; // RightParen
		let body: Block = self.parse_block()?;
		Ok(Statement::Fn { name, params, body })
	}

	pub fn parse_function_anonymous(&mut self) -> Result<Expression, Exception> {
		self.next_token(true)?; // FN
		let mut params: Vec<String> = Vec::new();

		if !self.ctoken.typer.is(TokenType::LeftParen) {
			let mut exception: Exception =
				Exception::not_runtime(Except::invalid_syntax("expected '('"));
			exception.push(ExceptionPoint::new(&self.module, self.ctoken.position.start.copy()));
			return Err(exception);
		}
		self.next_token(true)?; // LeftParen

		while !self.ctoken.typer.is(TokenType::RightParen) {
			if let TokenType::IDENTIFIER(argument) = self.ctoken.typer.clone() {
				params.push(argument);
				self.next_token(true)?; // IDENTIFIER
			}

			match &self.ctoken.typer {
				TokenType::COMMA => {
					if params.len() > 0 {
						self.next_token(true)?; // COMMA
					} else {
						let mut exception: Exception =
							Exception::not_runtime(Except::invalid_syntax("invalid syntax"));
						exception.push(ExceptionPoint::new(
							&self.module,
							self.ctoken.position.start.copy(),
						));
						return Err(exception);
					}
				},
				TokenType::RightParen => {},
				_ => {
					let mut exception: Exception =
						Exception::not_runtime(Except::invalid_syntax("expected ',' or ')'"));
					exception
						.push(ExceptionPoint::new(&self.module, self.ctoken.position.start.copy()));
					return Err(exception);
				},
			}
		}

		self.next_token(true)?; // RightParen
		let body: Block = self.parse_block()?;
		Ok(Expression::Fn { params, body })
	}
}
