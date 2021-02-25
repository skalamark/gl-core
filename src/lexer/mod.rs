// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

mod lidentifier;
mod lnumber;
mod lpunctuations;
mod lstring;
mod lwhitespace;

type ResultLexer = Result<(), Exception>;

pub struct Lexer {
	cchar: char,
	position: Position,
	eof_source: bool,
	source: Source,
	tokens_cache: Vec<Token>,
	module: String,
}

impl Lexer {
	pub fn new(source: Source, module: &String) -> Self {
		let mut lexer: Self = Self {
			cchar: '\0',
			position: Position::default(),
			eof_source: false,
			source,
			tokens_cache: Vec::new(),
			module: module.clone(),
		};
		lexer.next_char();
		lexer
	}

	pub fn get_module(&self) -> String {
		self.module.clone()
	}

	fn next_char(&mut self) {
		if self.cchar != '\0' {
			self.position.column += 1;
		}

		self.cchar = match self.source.next_char() {
			Some(c) => c,
			None => {
				self.eof_source = true;
				'\0'
			}
		};
	}

	fn is_eof_char(&self) -> bool {
		self.cchar == '\0'
	}

	fn push_token_in_cache(&mut self, token: Token) {
		self.tokens_cache.push(token);
	}

	pub fn next(&mut self) -> Result<Token, Exception> {
		if self.tokens_cache.len() > 0 {
			return Ok(self.tokens_cache.remove(0));
		}

		if self.eof_source && self.is_eof_char() {
			return Ok(Token::new(
				TokenType::EOF,
				TokenPosition::new(self.position.copy(), self.position.copy()),
			));
		}

		let result: Result<(), Exception> = match self.cchar {
			c if c.is_whitespace() => self.whitespace(),
			c if c.is_ascii_punctuation() => self.punctuations(),
			c if c.is_digit(10) => self.number(),
			c if c.is_alphabetic() => self.identifier_keyword(),
			_ => {
				let mut exception: Exception = Exception::new_not_runtime(Except::invalid_syntax(
					format!("invalid character '{}'", &self.cchar),
				));
				exception.push(ExceptionPoint::new(
					self.module.clone(),
					self.position.copy(),
				));
				Err(exception)
			}
		};

		if let Err(exception) = result {
			return Err(exception);
		}

		self.next()
	}

	pub fn run(&mut self) -> Result<Vec<Token>, Exception> {
		let mut tokens: Vec<Token> = vec![];
		let mut is_eof: bool = false;

		loop {
			if is_eof == true {
				break;
			}

			match self.next() {
				Ok(token) => {
					if token.typer.is(TokenType::EOF) {
						is_eof = true;
					}
					tokens.push(token)
				}
				Err(exception) => return Err(exception),
			}
		}

		Ok(tokens)
	}
}
