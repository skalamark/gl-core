// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn statement(&mut self, check_final: bool) -> Result<Statement, Exception> {
		self.next_while_newline()?;

		let mut statement: Statement = match &self.ctoken.typer {
			TokenType::LET => self.parse_let(),
			TokenType::FN if self.is_fn_statement_anonymous()? == "statement" => {
				self.parse_function()
			}
			TokenType::IMPORT => self.parse_import(),
			_ => {
				let expression: Expression = match self.parse_expression(Precedence::Lowest) {
					Ok(expression) => expression,
					Err(exception) => return Err(exception),
				};
				Ok(Statement::Expression(expression))
			}
		}?;

		if check_final {
			statement = self.parse_statement_final(statement)?
		}

		Ok(statement)
	}

	pub fn parse_statement_final(&mut self, statement: Statement) -> Result<Statement, Exception> {
		match &self.ctoken.typer {
			TokenType::EOF => {
				if let Statement::Expression(expression) = statement {
					return Ok(Statement::ExpressionReturn(expression));
				}
			}
			TokenType::SEMICOLON => {
				self.next_token(true)?;
			}
			TokenType::NEWLINE => {
				self.next_token(true)?;
				match &self.ctoken.typer {
					TokenType::SEMICOLON => {
						self.next_token(true)?;
					}
					_ => {
						if let Statement::Expression(expression) = statement {
							return Ok(Statement::ExpressionReturn(expression));
						}
					}
				}
			}
			_ => {
				let mut exception: Exception = Exception::new(
					Except::invalid_syntax(format!("expected ';', newline or eof")),
					false,
				);

				exception.push(ExceptionPoint::new(
					self.module.clone(),
					self.ctoken.position.start.copy(),
				));

				return Err(exception);
			}
		}

		return Ok(statement);
	}
}
