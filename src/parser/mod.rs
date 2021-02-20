// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

mod block;
mod call;
mod expression;
mod function;
mod hashmap;
mod if_;
mod index;
mod infix;
mod integer;
mod let_;
mod prefix;
mod statement;
mod vec;

pub struct Parser {
	ctoken: Token,
	ntoken: Token,
	lexer: Lexer,
	module: String,
}

impl Parser {
	pub fn new(lexer: Lexer) -> Self {
		let module: String = lexer.get_module();
		let mut parser: Self = Self {
			ctoken: Token::default(),
			ntoken: Token::default(),
			lexer,
			module,
		};

		let _ = parser.next_token(false);
		let _ = parser.next_token(true);

		parser
	}

	fn next_token(&mut self, while_newline: bool) -> Result<(), Exception> {
		self.ctoken = self.ntoken.copy();
		self.ntoken = self.lexer.next()?;

		if while_newline {
			self.next_while_newline()?;
		}

		Ok(())
	}

	fn next_while_newline(&mut self) -> Result<(), Exception> {
		while self.ctoken.typer.is(TokenType::NEWLINE) {
			self.next_token(false)?;
		}

		Ok(())
	}

	pub fn next(&mut self) -> Result<Option<Statement>, Exception> {
		if self.ctoken.typer.is(TokenType::EOF) {
			return Ok(None);
		}

		Ok(Some(self.statement(true)?))
	}

	pub fn run(&mut self) -> Result<AbstractSyntaxTree, Exception> {
		let mut ast: AbstractSyntaxTree = AbstractSyntaxTree::new();

		loop {
			ast.push(match self.next()? {
				Some(statement) => statement,
				None => break,
			});
		}

		Ok(ast)
	}
}
