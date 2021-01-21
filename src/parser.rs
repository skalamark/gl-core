// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::ast::{AbstractSyntaxTree, Expression, Literal, Statement};
use crate::error::{Exception, ExceptionError, ExceptionMain};
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

	fn parse_hashmap(
		&mut self, module: &String, program: &mut ProgramState,
	) -> Result<Literal, ExceptionMain> {
		self.next(true);
		let mut list: Vec<(Expression, Expression)> = Vec::new();

		while self.ctoken.typer != TokenType::RBrace {
			let key: Expression = match self.parse_expression(module, program) {
				Ok(key) => key,
				Err(exception) => return Err(exception),
			};

			if self.ctoken.typer != TokenType::COLON {
				let mut exception: ExceptionMain = ExceptionMain::new(
					ExceptionError::invalid_syntax(format!("expected ':'")),
					false,
				);
				exception.push(Exception::new(
					module.clone(),
					self.ctoken.position.start.copy(),
				));
				return Err(exception);
			}
			self.next(true);

			let value: Expression = match self.parse_expression(module, program) {
				Ok(value) => value,
				Err(exception) => return Err(exception),
			};

			list.push((key, value));
			self.next_while_newline();

			match &self.ctoken.typer {
				TokenType::COMMA => self.next(true),
				TokenType::RBrace => {}
				_ => {
					let mut exception: ExceptionMain = ExceptionMain::new(
						ExceptionError::invalid_syntax(format!("expected ',' or '}}'")),
						false,
					);
					exception.push(Exception::new(
						module.clone(),
						self.ctoken.position.start.copy(),
					));
					return Err(exception);
				}
			}
		}
		self.next(false);

		Ok(Literal::HashMap(list))
	}

	fn parse_vec(
		&mut self, module: &String, program: &mut ProgramState,
	) -> Result<Literal, ExceptionMain> {
		self.next(true);
		let mut list: Vec<Expression> = Vec::new();

		while self.ctoken.typer != TokenType::RBracket {
			let expression: Expression = match self.parse_expression(module, program) {
				Ok(expression) => expression,
				Err(exception) => return Err(exception),
			};
			list.push(expression);
			self.next_while_newline();

			match &self.ctoken.typer {
				TokenType::COMMA => self.next(true),
				TokenType::RBracket => {}
				_ => {
					let mut exception: ExceptionMain = ExceptionMain::new(
						ExceptionError::invalid_syntax(format!("expected ',' or ']'")),
						false,
					);
					exception.push(Exception::new(
						module.clone(),
						self.ctoken.position.start.copy(),
					));
					return Err(exception);
				}
			}
		}
		self.next(false);

		Ok(Literal::Vec(list))
	}

	fn parse_integer(
		&mut self, integer_literal: String, _: &String, _: &mut ProgramState,
	) -> Result<Literal, ExceptionMain> {
		Ok(Literal::Integer(
			BigInt::parse_bytes(integer_literal.as_bytes(), 10).unwrap(),
		))
	}

	fn parse_expression(
		&mut self, module: &String, program: &mut ProgramState,
	) -> Result<Expression, ExceptionMain> {
		// prefix
		let left: Expression = match self.ctoken.typer.clone() {
			t if t.is_eof() => {
				let mut exception: ExceptionMain = ExceptionMain::new(
					ExceptionError::unexpected_eof(format!("unexpected EOF while parsing")),
					false,
				);
				exception.push(Exception::new(
					module.clone(),
					self.ctoken.position.start.copy(),
				));
				return Err(exception);
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
			TokenType::LBracket => match self.parse_vec(module, program) {
				Ok(vec_literal) => Expression::Literal(vec_literal),
				Err(exception) => return Err(exception),
			},
			TokenType::LBrace => match self.parse_hashmap(module, program) {
				Ok(hashmap_literal) => Expression::Literal(hashmap_literal),
				Err(exception) => return Err(exception),
			},
			_ => {
				let mut exception: ExceptionMain = ExceptionMain::new(
					ExceptionError::invalid_syntax(format!("invalid token")),
					false,
				);
				exception.push(Exception::new(
					module.clone(),
					self.ctoken.position.start.copy(),
				));
				return Err(exception);
			}
		};

		Ok(left)
	}

	fn parse_let(
		&mut self, _: &mut AbstractSyntaxTree, module: &String, program: &mut ProgramState,
	) -> Result<Statement, ExceptionMain> {
		self.next(true);

		let name: String = match self.ctoken.typer.clone() {
			TokenType::IDENTIFIER(name) => name,
			_ => {
				let mut exception: ExceptionMain = ExceptionMain::new(
					ExceptionError::invalid_syntax(format!("expected identifier")),
					false,
				);
				exception.push(Exception::new(
					module.clone(),
					self.ctoken.position.start.copy(),
				));
				return Err(exception);
			}
		};
		self.next(true);

		match self.ctoken.typer.clone() {
			TokenType::ASSIGN => {}
			_ => {
				let mut exception: ExceptionMain = ExceptionMain::new(
					ExceptionError::invalid_syntax(format!("expected '='")),
					false,
				);
				exception.push(Exception::new(
					module.clone(),
					self.ctoken.position.start.copy(),
				));
				return Err(exception);
			}
		}
		self.next(true);

		let value: Expression = match self.parse_expression(module, program) {
			Ok(expression) => expression,
			Err(exception) => return Err(exception),
		};

		Ok(Statement::Let(name, value))
	}

	fn parse_statement(
		&mut self, ast: &mut AbstractSyntaxTree, module: &String, program: &mut ProgramState,
	) -> Result<(), ExceptionMain> {
		self.next_while_newline();

		match &self.ctoken.typer {
			t if t.is_eof() => return Ok(()),
			TokenType::LET => match self.parse_let(ast, module, program) {
				Ok(let_statement) => ast.push(let_statement),
				Err(exception) => return Err(exception),
			},
			_ => {
				let expression: Expression = match self.parse_expression(module, program) {
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
				match &self.ctoken.typer {
					TokenType::SEMICOLON => {
						self.next(true);
					}
					_ => {}
				}
			}
			_ => {
				let mut exception: ExceptionMain = ExceptionMain::new(
					ExceptionError::invalid_syntax(format!("expected ';', newline or eof")),
					false,
				);
				exception.push(Exception::new(
					module.clone(),
					self.ctoken.position.start.copy(),
				));
				return Err(exception);
			}
		}

		Ok(())
	}

	pub fn run(
		&mut self, tokens: Vec<Token>, module: &String, program: &mut ProgramState,
	) -> Result<AbstractSyntaxTree, ExceptionMain> {
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
