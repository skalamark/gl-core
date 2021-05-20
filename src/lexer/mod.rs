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
	nchar: char,
	eof: bool,
	position: Position,
	tokens_cache: Vec<Token>,
	source: Source,
	module: String,
}

impl Lexer {
	pub fn new<T: Into<String>>(source: Source, module: T) -> Self {
		let mut lexer: Self = Self {
			cchar: '\0',
			nchar: '\0',
			eof: false,
			position: Position::default(),
			source,
			tokens_cache: Vec::new(),
			module: module.into(),
		};
		lexer.next_char(); // '\0'
		lexer.next_char(); // '\0'
		lexer
	}

	pub fn get_module(&self) -> String { self.module.clone() }

	fn is_eof_char(&self) -> bool { self.cchar == '\0' }

	fn next_char(&mut self) {
		if self.cchar != '\0' {
			self.position.column += 1;
		}

		self.cchar = self.nchar;
		self.nchar = match self.source.next_char() {
			Some(c) => c,
			None => '\0',
		};
	}

	fn push_token(&mut self, token: Token) {
		if token.typer.is(TokenType::EOF) {
			self.eof = true
		}

		self.tokens_cache.push(token);
	}

	fn make_token_and_push(
		&mut self, typer: TokenType, position_start: Position, position_end: Position,
	) {
		self.push_token(Token::new(typer, TokenPosition::new(position_start, position_end)));
	}

	fn invalid_syntax_err(&self) -> ResultLexer {
		let mut exception: Exception = Exception::not_runtime(Except::invalid_syntax(format!(
			"invalid character '{}'",
			self.cchar
		)));
		exception.push(ExceptionPoint::new(&self.module, self.position.copy()));
		return Err(exception);
	}

	pub fn next(&mut self) -> Result<Token, Exception> {
		if self.tokens_cache.len() > 0 {
			return Ok(self.tokens_cache.remove(0));
		}

		if self.is_eof_char() {
			self.make_token_and_push(TokenType::EOF, self.position.copy(), self.position.copy());
			return self.next();
		}

		match self.cchar {
			c if c.is_whitespace() => self.lexe_whitespace()?,
			c if c.is_ascii_punctuation() => self.lexe_punctuations()?,
			c if c.is_digit(10) => self.lexe_number()?,
			c if c == '_' || c.is_alphabetic() => self.lexe_identifier_keyword()?,
			_ => self.invalid_syntax_err()?,
		};

		self.next()
	}

	pub fn run(&mut self) -> Result<Vec<Token>, Exception> {
		let mut tokens: Vec<Token> = vec![];

		loop {
			tokens.push(self.next()?);

			if self.is_eof_char() {
				if !self.eof {
					tokens.push(self.next()?);
				}
				break;
			}
		}

		Ok(tokens)
	}
}
