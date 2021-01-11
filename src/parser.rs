// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::ast::{AbstractSyntaxTree, Expression, Literal, Statement};
use crate::error::{AnyError, Error};
use crate::state::ProgramState;
use crate::token::{Token, TokenPosition, TokenType};

pub struct Parser {
	ctoken: Token,
	tokens: Vec<Token>,
}

impl Parser {
	pub fn new() -> Self {
		Self {
			ctoken: Token::new(TokenType::EOF, TokenPosition::default()),
			tokens: Vec::new(),
		}
	}

	fn next(&mut self) {
		if self.tokens.len() > 0 {
			self.ctoken = self.tokens.remove(0);
		} else {
			self.ctoken = Token::new(TokenType::EOF, self.ctoken.position.copy());
		}
	}

	fn parse_integer(
		&mut self, integer_literal: String, module: &String, program: &mut ProgramState,
	) -> Result<Literal, AnyError> {
		Ok(Literal::Integer(integer_literal.parse().unwrap()))
	}

	fn parse_expression(
		&mut self, ast: &mut AbstractSyntaxTree, module: &String, program: &mut ProgramState,
	) -> Result<Expression, AnyError> {
		let expression: Expression = match self.ctoken.typer.clone() {
			t if t.is_eof() => return Err(Error::invalid_syntax(format!("invalid token"))),
			TokenType::INTEGER(integer_literal) => {
				self.next();
				match self.parse_integer(integer_literal, module, program) {
					Ok(integer_literal) => Expression::Literal(integer_literal),
					Err(exception) => return Err(exception),
				}
			}

			_ => return Err(Error::invalid_syntax(format!("invalid token"))),
		};

		Ok(expression)
	}

	fn parse_statement(
		&mut self, ast: &mut AbstractSyntaxTree, module: &String, program: &mut ProgramState,
	) -> Result<(), AnyError> {
		match &self.ctoken.typer {
			t if t.is_eof() => {}

			_ => {
				let expression: Expression = match self.parse_expression(ast, module, program) {
					Ok(expression) => expression,
					Err(exception) => return Err(exception),
				};
				ast.push(Statement::Expression(expression));
			}
		}

		Ok(())
	}

	pub fn run(
		&mut self, tokens: Vec<Token>, module: &String, program: &mut ProgramState,
	) -> Result<AbstractSyntaxTree, AnyError> {
		let mut ast: AbstractSyntaxTree = AbstractSyntaxTree::new();
		self.tokens = tokens;
		self.next();

		loop {
			if self.ctoken.typer.is_eof() {
				break;
			}

			match self.parse_statement(&mut ast, module, program) {
				Ok(_) => {}
				Err(exception) => return Err(exception),
			}
		}

		Ok(ast)
	}
}
