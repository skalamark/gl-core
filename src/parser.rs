// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::ast::{
	AbstractSyntaxTree, Block, Expression, Infix, Literal, Precedence, Prefix, Statement,
};
use crate::error::{Exception, ExceptionError, ExceptionMain};
use crate::state::ProgramState;
use crate::token::{Token, TokenPosition, TokenType};
use num::BigInt;

pub struct Parser {
	ctoken: Token,
	ntoken: Token,
	tokens: Vec<Token>,
}

impl Parser {
	pub fn new() -> Self {
		Self {
			ctoken: Token::new(TokenType::EOF, TokenPosition::default()),
			ntoken: Token::new(TokenType::EOF, TokenPosition::default()),
			tokens: Vec::new(),
		}
	}

	fn next(&mut self, while_newline: bool) {
		self.ctoken = self.ntoken.clone();
		if self.tokens.len() > 0 {
			self.ntoken = self.tokens.remove(0);
		} else {
			self.ntoken = Token::new(TokenType::EOF, self.ctoken.position.copy());
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

	fn is_fn_statement_anonymous(&self, module: &String) -> Result<&str, ExceptionMain> {
		match &self.ntoken.typer {
			TokenType::IDENTIFIER(_) => return Ok("statement"),
			TokenType::LeftParen => return Ok("anonymous"),
			TokenType::NEWLINE => {}
			_ => {
				let mut exception: ExceptionMain = ExceptionMain::new(
					ExceptionError::invalid_syntax(format!("expected identifier or '('")),
					false,
				);
				exception.push(Exception::new(
					module.clone(),
					self.ntoken.position.start.copy(),
				));
				return Err(exception);
			}
		}

		for t in self.tokens.iter() {
			match &t.typer {
				TokenType::IDENTIFIER(_) => return Ok("statement"),
				TokenType::LeftParen => return Ok("anonymous"),
				TokenType::NEWLINE => {}
				_ => {
					let mut exception: ExceptionMain = ExceptionMain::new(
						ExceptionError::invalid_syntax(format!("expected identifier or '('")),
						false,
					);
					exception.push(Exception::new(module.clone(), t.position.start.copy()));
					return Err(exception);
				}
			}
		}

		let mut exception: ExceptionMain = ExceptionMain::new(
			ExceptionError::invalid_syntax(format!("expected identifier or '('")),
			false,
		);
		exception.push(Exception::new(
			module.clone(),
			self.ntoken.position.start.copy(),
		));
		return Err(exception);
	}

	fn parse_function_anonymous(
		&mut self, module: &String, program: &mut ProgramState,
	) -> Result<Expression, ExceptionMain> {
		self.next(true);
		let mut params: Vec<String> = Vec::new();

		match &self.ctoken.typer {
			TokenType::LeftParen => {}
			_ => {
				let mut exception: ExceptionMain = ExceptionMain::new(
					ExceptionError::invalid_syntax(format!("expected '('")),
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

		while self.ctoken.typer != TokenType::RightParen {
			match &self.ctoken.typer {
				TokenType::IDENTIFIER(arg) => {
					params.push(arg.clone());
				}
				_ => {}
			}
			self.next(true);

			match &self.ctoken.typer {
				TokenType::COMMA => self.next(true),
				TokenType::RightParen => {}
				_ => {
					let mut exception: ExceptionMain = ExceptionMain::new(
						ExceptionError::invalid_syntax(format!("expected ',' or ')'")),
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
		self.next(true);

		let body: Block = match self.parse_block(module, program) {
			Ok(block) => block,
			Err(exception) => return Err(exception),
		};

		Ok(Expression::Fn { params, body })
	}

	fn parse_call(
		&mut self, function: Expression, module: &String, program: &mut ProgramState,
	) -> Result<Expression, ExceptionMain> {
		self.next(true);
		let mut arguments: Vec<Expression> = Vec::new();

		while self.ctoken.typer != TokenType::RightParen {
			let expression: Expression =
				match self.parse_expression(Precedence::Lowest, module, program) {
					Ok(expression) => expression,
					Err(exception) => return Err(exception),
				};
			arguments.push(expression);
			self.next_while_newline();

			match &self.ctoken.typer {
				TokenType::COMMA => self.next(true),
				TokenType::RightParen => {}
				_ => {
					let mut exception: ExceptionMain = ExceptionMain::new(
						ExceptionError::invalid_syntax(format!("expected ',' or ')'")),
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

		Ok(Expression::Call {
			function: Box::new(function),
			arguments,
		})
	}

	fn parse_infix(
		&mut self, left: Expression, module: &String, program: &mut ProgramState,
	) -> Result<Expression, ExceptionMain> {
		let mut infix: Infix = Infix::Plus;

		match &self.ctoken.typer {
			TokenType::PLUS => infix = Infix::Plus,
			TokenType::MINUS => infix = Infix::Minus,
			TokenType::ASTERISK => infix = Infix::Multiply,
			TokenType::SLASH => infix = Infix::Divide,
			TokenType::EQUAL => infix = Infix::Equal,
			TokenType::NotEqual => infix = Infix::NotEqual,
			TokenType::LessThan => infix = Infix::LessThan,
			TokenType::LessThanEqual => infix = Infix::LessThanEqual,
			TokenType::GreaterThan => infix = Infix::GreaterThan,
			TokenType::GreaterThanEqual => infix = Infix::GreaterThanEqual,
			_ => {}
		};
		let precedence: Precedence = Precedence::from_token(&self.ctoken);
		self.next(true);

		match self.parse_expression(precedence, module, program) {
			Ok(expression) => Ok(Expression::Infix(
				infix,
				Box::new(left),
				Box::new(expression),
			)),
			Err(exception) => return Err(exception),
		}
	}

	fn parse_prefix(
		&mut self, module: &String, program: &mut ProgramState,
	) -> Result<Expression, ExceptionMain> {
		let mut prefix: Prefix = Prefix::Not;

		match &self.ctoken.typer {
			TokenType::BANG => prefix = Prefix::Not,
			TokenType::PLUS => prefix = Prefix::Plus,
			TokenType::MINUS => prefix = Prefix::Minus,
			_ => {}
		};
		self.next(true);

		match self.parse_expression(Precedence::Prefix, module, program) {
			Ok(expression) => Ok(Expression::Prefix(prefix, Box::new(expression))),
			Err(exception) => return Err(exception),
		}
	}

	fn parse_hashmap(
		&mut self, module: &String, program: &mut ProgramState,
	) -> Result<Literal, ExceptionMain> {
		self.next(true);
		let mut list: Vec<(Expression, Expression)> = Vec::new();

		while self.ctoken.typer != TokenType::RightBrace {
			let key: Expression = match self.parse_expression(Precedence::Lowest, module, program) {
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

			let value: Expression = match self.parse_expression(Precedence::Lowest, module, program)
			{
				Ok(value) => value,
				Err(exception) => return Err(exception),
			};

			list.push((key, value));
			self.next_while_newline();

			match &self.ctoken.typer {
				TokenType::COMMA => self.next(true),
				TokenType::RightBrace => {}
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

		while self.ctoken.typer != TokenType::RightBracket {
			let expression: Expression =
				match self.parse_expression(Precedence::Lowest, module, program) {
					Ok(expression) => expression,
					Err(exception) => return Err(exception),
				};
			list.push(expression);
			self.next_while_newline();

			match &self.ctoken.typer {
				TokenType::COMMA => self.next(true),
				TokenType::RightBracket => {}
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
		&mut self, precedence: Precedence, module: &String, program: &mut ProgramState,
	) -> Result<Expression, ExceptionMain> {
		// prefix
		let mut left: Expression = match self.ctoken.typer.clone() {
			t if t.is(crate::token::TokenType::EOF) => {
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
			TokenType::LeftBracket => match self.parse_vec(module, program) {
				Ok(vec_literal) => Expression::Literal(vec_literal),
				Err(exception) => return Err(exception),
			},
			TokenType::LeftBrace => match self.parse_hashmap(module, program) {
				Ok(hashmap_literal) => Expression::Literal(hashmap_literal),
				Err(exception) => return Err(exception),
			},
			TokenType::BANG | TokenType::PLUS | TokenType::MINUS => {
				match self.parse_prefix(module, program) {
					Ok(expression) => expression,
					Err(exception) => return Err(exception),
				}
			}
			TokenType::FN => match self.parse_function_anonymous(module, program) {
				Ok(function_anonymous) => function_anonymous,
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

		// infix
		while precedence < Precedence::from_token(&self.ctoken) {
			match &self.ctoken.typer {
				TokenType::PLUS
				| TokenType::MINUS
				| TokenType::ASTERISK
				| TokenType::SLASH
				| TokenType::EQUAL
				| TokenType::NotEqual
				| TokenType::LessThan
				| TokenType::LessThanEqual
				| TokenType::GreaterThan
				| TokenType::GreaterThanEqual => {
					left = match self.parse_infix(left, module, program) {
						Ok(expression) => expression,
						Err(exception) => return Err(exception),
					};
				}
				TokenType::LeftParen => {
					left = match self.parse_call(left, module, program) {
						Ok(expression) => expression,
						Err(exception) => return Err(exception),
					};
				}
				_ => return Ok(left),
			}
		}

		Ok(left)
	}

	fn parse_function(
		&mut self, _: &mut AbstractSyntaxTree, module: &String, program: &mut ProgramState,
	) -> Result<Statement, ExceptionMain> {
		self.next(true);
		let name: String = match self.ctoken.typer.clone() {
			TokenType::IDENTIFIER(name) => name,
			_ => {
				let mut exception: ExceptionMain = ExceptionMain::new(
					ExceptionError::invalid_syntax(format!("expected identifier'")),
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
		let mut params: Vec<String> = Vec::new();

		match &self.ctoken.typer {
			TokenType::LeftParen => {}
			_ => {
				let mut exception: ExceptionMain = ExceptionMain::new(
					ExceptionError::invalid_syntax(format!("expected '('")),
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

		while self.ctoken.typer != TokenType::RightParen {
			match &self.ctoken.typer {
				TokenType::IDENTIFIER(arg) => {
					params.push(arg.clone());
				}
				_ => {}
			}
			self.next(true);

			match &self.ctoken.typer {
				TokenType::COMMA => self.next(true),
				TokenType::RightParen => {}
				_ => {
					let mut exception: ExceptionMain = ExceptionMain::new(
						ExceptionError::invalid_syntax(format!("expected ',' or ')'")),
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
		self.next(true);

		let body: Block = match self.parse_block(module, program) {
			Ok(block) => block,
			Err(exception) => return Err(exception),
		};

		Ok(Statement::Fn { name, params, body })
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

		let value: Expression = match self.parse_expression(Precedence::Lowest, module, program) {
			Ok(expression) => expression,
			Err(exception) => return Err(exception),
		};

		Ok(Statement::Let(name, value))
	}

	fn parse_block(
		&mut self, module: &String, program: &mut ProgramState,
	) -> Result<Block, ExceptionMain> {
		let mut block: AbstractSyntaxTree = AbstractSyntaxTree::new();

		match &self.ctoken.typer {
			TokenType::LeftBrace => {}
			_ => {
				let mut exception: ExceptionMain = ExceptionMain::new(
					ExceptionError::invalid_syntax(format!("expected '{{'")),
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

		while self.ctoken.typer != TokenType::RightBrace {
			match self.parse_statement(&mut block, module, program) {
				Ok(_) => {}
				Err(exception) => return Err(exception),
			}
		}
		self.next(false);

		Ok(Block(block.statements))
	}

	fn parse_statement(
		&mut self, ast: &mut AbstractSyntaxTree, module: &String, program: &mut ProgramState,
	) -> Result<(), ExceptionMain> {
		self.next_while_newline();

		match &self.ctoken.typer {
			t if t.is(crate::token::TokenType::EOF) => return Ok(()),
			TokenType::LET => match self.parse_let(ast, module, program) {
				Ok(let_statement) => ast.push(let_statement),
				Err(exception) => return Err(exception),
			},
			TokenType::FN
				if {
					match self.is_fn_statement_anonymous(module) {
						Ok(typer) => typer,
						Err(exception) => return Err(exception),
					}
				} == "statement" =>
			{
				match self.parse_function(ast, module, program) {
					Ok(fn_statement) => ast.push(fn_statement),
					Err(exception) => return Err(exception),
				}
			}
			_ => {
				let expression: Expression =
					match self.parse_expression(Precedence::Lowest, module, program) {
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
		self.next(false);
		self.next(true);

		loop {
			if self.ctoken.typer.is(crate::token::TokenType::EOF) {
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
