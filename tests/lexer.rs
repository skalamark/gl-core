// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

extern crate gl_core;

use gl_core::error::AnyError;
use gl_core::lexer::Lexer;
use gl_core::position::Position;
use gl_core::state::ProgramState;
use gl_core::token::{Token, TokenPosition};

#[test]
fn new() {
	let _lexer: Lexer = Lexer::new();
}

#[test]
fn run_empty() {
	let mut lexer: Lexer = Lexer::new();
	let source: String = format!("");
	let module: String = format!("crate");
	let mut program: ProgramState = ProgramState::new();

	let expected_tokens: Vec<Token> = vec![Token::new(
		gl_core::token::TokenType::EOF,
		TokenPosition::new(Position::new(0, 0), Position::new(0, 0)),
	)];
	let rtokens: Result<Vec<Token>, AnyError> = lexer.run(source, &module, &mut program);

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_whitespaces() {
	let mut lexer: Lexer = Lexer::new();
	let source: String = format!("  	\r\n\t ");
	let module: String = format!("crate");
	let mut program: ProgramState = ProgramState::new();

	let expected_tokens: Vec<Token> = vec![Token::new(
		gl_core::token::TokenType::EOF,
		TokenPosition::new(Position::new(2, 1), Position::new(2, 1)),
	)];
	let rtokens: Result<Vec<Token>, AnyError> = lexer.run(source, &module, &mut program);

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_digits() {
	let mut lexer: Lexer = Lexer::new();
	let source: String = format!("1234567890");
	let module: String = format!("crate");
	let mut program: ProgramState = ProgramState::new();

	let expected_tokens: Vec<Token> = vec![
		Token::new(
			gl_core::token::TokenType::INTEGER(format!("1234567890")),
			TokenPosition::new(Position::new(1, 0), Position::new(10, 0)),
		),
		Token::new(
			gl_core::token::TokenType::EOF,
			TokenPosition::new(Position::new(10, 0), Position::new(10, 0)),
		),
	];
	let rtokens: Result<Vec<Token>, AnyError> = lexer.run(source, &module, &mut program);

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}
