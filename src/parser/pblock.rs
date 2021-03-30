// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn parse_block(&mut self) -> Result<Block, Exception> {
		let mut block: AbstractSyntaxTree = AbstractSyntaxTree::new();

		if self.ctoken.typer.is(TokenType::LeftBrace) {
			self.next_token(true)?; // LeftBrace

			while !self.ctoken.typer.is(TokenType::RightBrace) {
				let mut statement: Statement = self.parse_statement(false)?;

				if self.ctoken.typer.is(TokenType::RightBrace) {
					if let Statement::Expression(expression) = statement {
						statement = Statement::ExpressionReturn(expression);
					}
				} else {
					statement = self.parse_statement_final(statement)?;
				}

				block.push(statement)
			}

			self.next_token(false)?; // RightBrace
			return Ok(Block(block.statements));
		}

		let mut exception: Exception =
			Exception::not_runtime(Except::invalid_syntax("expected '{{'"));
		exception.push(ExceptionPoint::new(&self.module, self.ctoken.position.start.copy()));
		Err(exception)
	}
}
