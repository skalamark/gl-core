// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn parse_block(&mut self) -> Result<Block, Exception> {
		let mut block: AbstractSyntaxTree = AbstractSyntaxTree::new();

		if !self.ctoken.typer.is(TokenType::LeftBrace) {
			let mut exception: Exception =
				Exception::new(Except::invalid_syntax(format!("expected '{{'")), false);

			exception.push(ExceptionPoint::new(
				self.module.clone(),
				self.ctoken.position.start.copy(),
			));

			return Err(exception);
		}
		self.next_token(true)?;

		while !self.ctoken.typer.is(TokenType::RightBrace) {
			block.push(self.statement(false)?);

			let is_eof: bool = self.ctoken.typer.is(TokenType::RightBrace);

			if is_eof {
				let last_statement: Statement = block.statements.remove(block.statements.len() - 1);
				if let Statement::Expression(expression) = last_statement {
					block.push(Statement::ExpressionReturn(expression));
				}
			} else {
				let last_statement: Statement = block.statements.remove(block.statements.len() - 1);
				block.push(self.parse_statement_final(last_statement)?);
			}
		}

		self.next_token(false)?;
		Ok(Block(block.statements))
	}
}
