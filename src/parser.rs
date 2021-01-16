// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::ast::{AbstractSyntaxTree, Expression, Literal, Statement};
use crate::error::{AnyError, Error, Exception};
use crate::state::ProgramState;
use crate::token::{Token, TokenPosition, TokenType};
use num::BigInt;

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

	fn next(&mut self, while_newline: bool) {
		if self.tokens.len() > 0 {
			self.ctoken = self.tokens.remove(0);
		} else {
			self.ctoken = Token::new(TokenType::EOF, self.ctoken.position.copy());
		}

		if while_newline {
			self.next_while_newline();
		}
	}

	fn next_while_newline(&mut self) {
		loop {
			match &self.ctoken.typer {
				TokenType::NEWLINE => self.next(true),
				_ => break,
			}
		}
	}

	fn parse_integer(
		&mut self, integer_literal: String, module: &String, program: &mut ProgramState,
	) -> Result<Literal, AnyError> {
		Ok(Literal::Integer(
			BigInt::parse_bytes(integer_literal.as_bytes(), 10).unwrap(),
		))
	}

	fn parse_expression(
		&mut self, ast: &mut AbstractSyntaxTree, module: &String, program: &mut ProgramState,
	) -> Result<Expression, AnyError> {
		let expression: Expression = match self.ctoken.typer.clone() {
			t if t.is_eof() => {
				return Err(Exception::new(
					module.clone(),
					self.ctoken.position.start.copy(),
					Error::unexpected_eof(format!("unexpected EOF while parsing")),
				));
			}
			TokenType::IDENTIFIER(identifier) => {
				self.next(false);
				Expression::Identifier(identifier)
			}
			TokenType::NULL => {
				self.next(false);
				Expression::Literal(Literal::Null)
			}
			TokenType::INTEGER(integer_literal) => {
				self.next(false);
				match self.parse_integer(integer_literal, module, program) {
					Ok(integer_literal) => Expression::Literal(integer_literal),
					Err(exception) => return Err(exception),
				}
			}
			TokenType::BOOLEAN(boolean_literal) => {
				self.next(false);
				Expression::Literal(Literal::Boolean(boolean_literal))
			}
			TokenType::STRING(string_literal) => {
				self.next(false);
				Expression::Literal(Literal::String(string_literal))
			}
			_ => {
				return Err(Exception::new(
					module.clone(),
					self.ctoken.position.start.copy(),
					Error::invalid_syntax(format!("invalid token")),
				));
			}
		};

		Ok(expression)
	}

	fn parse_let(
		&mut self, ast: &mut AbstractSyntaxTree, module: &String, program: &mut ProgramState,
	) -> Result<Statement, AnyError> {
		self.next(true);

		let name: String = match self.ctoken.typer.clone() {
			TokenType::IDENTIFIER(name) => name,
			_ => {
				return Err(Exception::new(
					module.clone(),
					self.ctoken.position.start.copy(),
					Error::invalid_syntax(format!("expected identifier")),
				));
			}
		};
		self.next(true);

		match self.ctoken.typer.clone() {
			TokenType::ASSIGN => {}
			_ => {
				return Err(Exception::new(
					module.clone(),
					self.ctoken.position.start.copy(),
					Error::invalid_syntax(format!("expected '='")),
				));
			}
		}
		self.next(true);

		let value: Expression = match self.parse_expression(ast, module, program) {
			Ok(expression) => expression,
			Err(exception) => return Err(exception),
		};

		Ok(Statement::Let(name, value))
	}

	fn parse_statement(
		&mut self, ast: &mut AbstractSyntaxTree, module: &String, program: &mut ProgramState,
	) -> Result<(), AnyError> {
		self.next_while_newline();

		match &self.ctoken.typer {
			t if t.is_eof() => return Ok(()),
			TokenType::LET => match self.parse_let(ast, module, program) {
				Ok(let_statement) => ast.push(let_statement),
				Err(exception) => return Err(exception),
			},
			_ => {
				let expression: Expression = match self.parse_expression(ast, module, program) {
					Ok(expression) => expression,
					Err(exception) => return Err(exception),
				};
				ast.push(Statement::Expression(expression));
			}
		}

		match &self.ctoken.typer {
			TokenType::EOF => {}
			TokenType::SEMICOLON => {
				self.next(true);
			}
			TokenType::NEWLINE => {
				self.next(true);
			}
			_ => {
				return Err(Exception::new(
					module.clone(),
					self.ctoken.position.start.copy(),
					Error::invalid_syntax(format!("expected ';', newline or eof")),
				))
			}
		}

		Ok(())
	}

	pub fn run(
		&mut self, tokens: Vec<Token>, module: &String, program: &mut ProgramState,
	) -> Result<AbstractSyntaxTree, AnyError> {
		let mut ast: AbstractSyntaxTree = AbstractSyntaxTree::new();
		self.tokens = tokens;
		self.next(true);

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
