// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

mod patom;
mod pblock;
mod pcall;
mod pexpression;
mod pfunction;
mod phashmap;
mod pimport;
mod pindex;
mod pinfix;
mod plet;
mod pnumber;
mod pprefix;
mod pstatement;
mod ptuple;
mod pvec;

pub struct Parser {
	ctoken: Token,
	ntoken: Token,
	tokens_cache: Vec<Token>,
	lexer: Lexer,
	module: String,
}

impl Parser {
	pub fn new(lexer: Lexer) -> Result<Self, Exception> {
		let module: String = lexer.get_module();
		let mut parser: Self = Self {
			ctoken: Token::default(),
			ntoken: Token::default(),
			tokens_cache: Vec::new(),
			lexer,
			module,
		};

		let _ = parser.next_token(false)?; // EOF
		let _ = parser.next_token(true)?; // EOF

		Ok(parser)
	}

	fn next_token(&mut self, while_newline: bool) -> Result<(), Exception> {
		self.ctoken = self.ntoken.copy();

		if self.tokens_cache.len() > 0 {
			self.ntoken = self.tokens_cache.remove(0);
		} else {
			self.ntoken = self.lexer.next()?;
		}

		if while_newline {
			self.next_while_newline()?;
		}

		Ok(())
	}

	fn next_token_from_lexer(&mut self) -> Result<Token, Exception> { self.lexer.next() }

	fn next_newline(&mut self) -> Result<(), Exception> {
		while !self.ctoken.typer.is(TokenType::NEWLINE) && !self.ctoken.typer.is(TokenType::EOF) {
			self.next_token(false)?; // all except NEWLINE and EOF
		}

		Ok(())
	}

	fn next_while_newline(&mut self) -> Result<(), Exception> {
		while self.ctoken.typer.is(TokenType::NEWLINE) {
			self.next_token(false)?; // NEWLINE
		}

		Ok(())
	}

	pub fn next(&mut self) -> Result<Option<Statement>, Exception> {
		self.next_while_newline()?;

		if self.ctoken.typer.is(TokenType::EOF) {
			return Ok(None);
		}

		Ok(Some(self.parse_statement(true)?))
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
