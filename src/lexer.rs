// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::error::{Exception, ExceptionError, ExceptionMain};
use crate::position::Position;
use crate::state::ProgramState;
use crate::token::Token;
use crate::token::TokenPosition;

type ResultLexer = Result<(), ExceptionMain>;

pub struct Lexer {
	cchar: char,
	chars: Vec<char>,
	position: Position,
}

impl Lexer {
	pub fn new() -> Self {
		Self {
			cchar: '\0',
			chars: Vec::new(),
			position: Position::default(),
		}
	}

	fn next(&mut self) {
		if self.chars.len() > 0 {
			if self.cchar != '\0' {
				self.position.column += 1;
			}
			self.cchar = self.chars.remove(0);
		} else {
			if self.cchar != '\0' {
				self.position.column += 1;
			}
			self.cchar = '\0';
		}
	}

	fn lexe_string(
		&mut self, tokens: &mut Vec<Token>, module: &String, _: &mut ProgramState,
	) -> ResultLexer {
		let mut string_literal: String = String::new();
		let position_start: Position = self.position.copy();
		self.next();

		while self.cchar != '\0' && self.cchar != '"' {
			string_literal.push(self.cchar);
			self.next()
		}

		if self.cchar != '"' {
			let mut exception = ExceptionMain::new(
				ExceptionError::unexpected_eof(format!("EOL while scanning string literal")),
				false,
			);
			exception.push(Exception::new(module.clone(), self.position.copy()));
			return Err(exception);
		}
		self.next();

		tokens.push(Token::new(
			crate::token::TokenType::STRING(string_literal),
			TokenPosition::new(position_start, self.position.copy()),
		));

		Ok(())
	}

	fn lexe_identifier_keyword(
		&mut self, tokens: &mut Vec<Token>, _: &String, _: &mut ProgramState,
	) -> ResultLexer {
		let mut identifier_literal: String = String::new();
		let position_start: Position = self.position.copy();

		while self.cchar != '\0'
			&& (self.cchar.is_alphabetic() || self.cchar.is_digit(10) || self.cchar == '_')
		{
			identifier_literal.push(self.cchar);
			self.next()
		}

		match identifier_literal.clone() {
			i if i == "null" => {
				tokens.push(Token::new(
					crate::token::TokenType::NULL,
					TokenPosition::new(position_start, self.position.copy()),
				));
			}
			i if i == "true" => {
				tokens.push(Token::new(
					crate::token::TokenType::BOOLEAN(true),
					TokenPosition::new(position_start, self.position.copy()),
				));
			}
			i if i == "false" => {
				tokens.push(Token::new(
					crate::token::TokenType::BOOLEAN(false),
					TokenPosition::new(position_start, self.position.copy()),
				));
			}
			i if i == "let" => {
				tokens.push(Token::new(
					crate::token::TokenType::LET,
					TokenPosition::new(position_start, self.position.copy()),
				));
			}
			i if i == "fn" => {
				tokens.push(Token::new(
					crate::token::TokenType::FN,
					TokenPosition::new(position_start, self.position.copy()),
				));
			}
			identifier => {
				tokens.push(Token::new(
					crate::token::TokenType::IDENTIFIER(identifier),
					TokenPosition::new(position_start, self.position.copy()),
				));
			}
		}
		Ok(())
	}

	fn lexe_digits(
		&mut self, tokens: &mut Vec<Token>, _: &String, _: &mut ProgramState,
	) -> ResultLexer {
		let mut digits_literal: String = String::new();
		let position_start: Position = self.position.copy();

		while self.cchar != '\0' && self.cchar.is_digit(10) {
			digits_literal.push(self.cchar);
			self.next()
		}

		tokens.push(Token::new(
			crate::token::TokenType::INTEGER(digits_literal),
			TokenPosition::new(position_start, self.position.copy()),
		));

		Ok(())
	}

	fn lexe_punctuations(
		&mut self, tokens: &mut Vec<Token>, module: &String, program: &mut ProgramState,
	) -> ResultLexer {
		let position_start: Position = self.position.copy();

		match self.cchar {
			c if c == '"' => return self.lexe_string(tokens, module, program),

			c if c == '+' => {
				self.next();
				tokens.push(Token::new(
					crate::token::TokenType::PLUS,
					TokenPosition::new(position_start, self.position.copy()),
				));
			}
			c if c == '-' => {
				self.next();
				tokens.push(Token::new(
					crate::token::TokenType::MINUS,
					TokenPosition::new(position_start, self.position.copy()),
				));
			}
			c if c == '*' => {
				self.next();
				tokens.push(Token::new(
					crate::token::TokenType::MULTIPLY,
					TokenPosition::new(position_start, self.position.copy()),
				));
			}
			c if c == '/' => {
				self.next();
				tokens.push(Token::new(
					crate::token::TokenType::DIVIDE,
					TokenPosition::new(position_start, self.position.copy()),
				));
			}
			c if c == '=' => {
				self.next();
				if self.cchar == '=' {
					self.next();
					tokens.push(Token::new(
						crate::token::TokenType::EQUAL,
						TokenPosition::new(position_start, self.position.copy()),
					));
				} else {
					tokens.push(Token::new(
						crate::token::TokenType::ASSIGN,
						TokenPosition::new(position_start, self.position.copy()),
					));
				}
			}
			c if c == '!' => {
				self.next();
				if self.cchar == '=' {
					self.next();
					tokens.push(Token::new(
						crate::token::TokenType::NotEqual,
						TokenPosition::new(position_start, self.position.copy()),
					));
				} else {
					tokens.push(Token::new(
						crate::token::TokenType::NOT,
						TokenPosition::new(position_start, self.position.copy()),
					));
				}
			}
			c if c == '>' => {
				self.next();
				if self.cchar == '=' {
					self.next();
					tokens.push(Token::new(
						crate::token::TokenType::GreaterThanEqual,
						TokenPosition::new(position_start, self.position.copy()),
					));
				} else {
					tokens.push(Token::new(
						crate::token::TokenType::GreaterThan,
						TokenPosition::new(position_start, self.position.copy()),
					));
				}
			}
			c if c == '<' => {
				self.next();
				if self.cchar == '=' {
					self.next();
					tokens.push(Token::new(
						crate::token::TokenType::LessThanEqual,
						TokenPosition::new(position_start, self.position.copy()),
					));
				} else {
					tokens.push(Token::new(
						crate::token::TokenType::LessThan,
						TokenPosition::new(position_start, self.position.copy()),
					));
				}
			}

			c if c == '.' => {
				self.next();
				tokens.push(Token::new(
					crate::token::TokenType::DOT,
					TokenPosition::new(position_start, self.position.copy()),
				));
			}
			c if c == ',' => {
				self.next();
				tokens.push(Token::new(
					crate::token::TokenType::COMMA,
					TokenPosition::new(position_start, self.position.copy()),
				));
			}
			c if c == ';' => {
				self.next();
				tokens.push(Token::new(
					crate::token::TokenType::SEMICOLON,
					TokenPosition::new(position_start, self.position.copy()),
				));
			}
			c if c == ':' => {
				self.next();
				tokens.push(Token::new(
					crate::token::TokenType::COLON,
					TokenPosition::new(position_start, self.position.copy()),
				));
			}
			c if c == '(' => {
				self.next();
				tokens.push(Token::new(
					crate::token::TokenType::LParen,
					TokenPosition::new(position_start, self.position.copy()),
				));
			}
			c if c == ')' => {
				self.next();
				tokens.push(Token::new(
					crate::token::TokenType::RParen,
					TokenPosition::new(position_start, self.position.copy()),
				));
			}
			c if c == '[' => {
				self.next();
				tokens.push(Token::new(
					crate::token::TokenType::LBracket,
					TokenPosition::new(position_start, self.position.copy()),
				));
			}
			c if c == ']' => {
				self.next();
				tokens.push(Token::new(
					crate::token::TokenType::RBracket,
					TokenPosition::new(position_start, self.position.copy()),
				));
			}
			c if c == '{' => {
				self.next();
				tokens.push(Token::new(
					crate::token::TokenType::LBrace,
					TokenPosition::new(position_start, self.position.copy()),
				));
			}
			c if c == '}' => {
				self.next();
				tokens.push(Token::new(
					crate::token::TokenType::RBrace,
					TokenPosition::new(position_start, self.position.copy()),
				));
			}
			_ => {
				let mut exception = ExceptionMain::new(
					ExceptionError::invalid_syntax(format!("invalid character '{}'", &self.cchar)),
					false,
				);
				exception.push(Exception::new(module.clone(), self.position.copy()));
				return Err(exception);
			}
		}

		Ok(())
	}

	fn lexe_whitespace(
		&mut self, tokens: &mut Vec<Token>, _: &String, _: &mut ProgramState,
	) -> ResultLexer {
		let mut position_start: Position = self.position.copy();

		while self.cchar != '\0' && self.cchar.is_whitespace() {
			if self.cchar == '\n' {
				self.position.column += 1;
				tokens.push(Token::new(
					crate::token::TokenType::NEWLINE,
					TokenPosition::new(position_start.copy(), self.position.copy()),
				));
				self.next();
				self.position.column = 0;
				self.position.line += 1;
				continue;
			}
			self.next();
			position_start = self.position.copy();
		}

		Ok(())
	}

	fn lexe_token(
		&mut self, tokens: &mut Vec<Token>, module: &String, program: &mut ProgramState,
	) -> ResultLexer {
		if self.chars.len() == 0 && self.cchar == '\0' {
			self.next();
			tokens.push(Token::new(
				crate::token::TokenType::EOF,
				TokenPosition::new(self.position.copy(), self.position.copy()),
			));
			return Ok(());
		}

		match self.cchar {
			c if c.is_whitespace() => match self.lexe_whitespace(tokens, module, program) {
				Ok(_) => {}
				Err(exception) => return Err(exception),
			},
			c if c.is_ascii_punctuation() => {
				match self.lexe_punctuations(tokens, module, program) {
					Ok(_) => {}
					Err(exception) => return Err(exception),
				}
			}

			c if c.is_digit(10) => match self.lexe_digits(tokens, module, program) {
				Ok(_) => {}
				Err(exception) => return Err(exception),
			},
			c if c.is_alphabetic() => match self.lexe_identifier_keyword(tokens, module, program) {
				Ok(_) => {}
				Err(exception) => return Err(exception),
			},
			_ => {
				let mut exception = ExceptionMain::new(
					ExceptionError::invalid_syntax(format!("invalid character '{}'", &self.cchar)),
					false,
				);
				exception.push(Exception::new(module.clone(), self.position.copy()));
				return Err(exception);
			}
		}

		Ok(())
	}

	pub fn run(
		&mut self, source: String, module: &String, program: &mut ProgramState,
	) -> Result<Vec<Token>, ExceptionMain> {
		self.chars = source.chars().collect();
		let mut tokens: Vec<Token> = Vec::new();
		self.next();

		loop {
			if let Some(last_token) = tokens.last() {
				if last_token.typer.is_eof() {
					break;
				}
			}

			match self.lexe_token(&mut tokens, module, program) {
				Ok(_) => {}
				Err(exception) => return Err(exception),
			}
		}

		Ok(tokens)
	}
}
