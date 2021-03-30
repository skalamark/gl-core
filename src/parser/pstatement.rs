// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn parse_statement(&mut self, check_final: bool) -> Result<Statement, Exception> {
		self.next_while_newline()?;

		let mut statement: Statement = match self.ctoken.typer.clone() {
			TokenType::LET => self.parse_let()?,
			TokenType::FN if self.is_fn_statement_anonymous()? == "statement" =>
				self.parse_function()?,
			TokenType::IMPORT => self.parse_import()?,
			_ => Statement::Expression(self.parse_expression(Precedence::Lowest)?),
		};

		if check_final {
			statement = self.parse_statement_final(statement)?;
		}
		Ok(statement)
	}

	pub fn parse_statement_final(&mut self, statement: Statement) -> Result<Statement, Exception> {
		match &self.ctoken.typer {
			TokenType::EOF =>
				if let Statement::Expression(expression) = statement {
					return Ok(Statement::ExpressionReturn(expression));
				},
			TokenType::SEMICOLON => self.next_token(true)?, // SEMICOLON
			TokenType::NEWLINE => {
				self.next_token(true)?; // NEWLINE
				match &self.ctoken.typer {
					TokenType::SEMICOLON => {
						self.next_token(true)?; // SEMICOLON
					},
					_ =>
						if let Statement::Expression(expression) = statement {
							return Ok(Statement::ExpressionReturn(expression));
						},
				}
			},
			_ => {
				let mut exception: Exception =
					Exception::not_runtime(Except::invalid_syntax("expected ';', newline or eof"));
				exception
					.push(ExceptionPoint::new(&self.module, self.ctoken.position.start.copy()));
				return Err(exception);
			},
		}

		return Ok(statement);
	}
}
