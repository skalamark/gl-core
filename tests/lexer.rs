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
	let module: String = format!("tests/lexer/empty");
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
	let module: String = format!("tests/lexer/whitespaces");
	let mut program: ProgramState = ProgramState::new();

	let expected_tokens: Vec<Token> = vec![
		Token::new(
			gl_core::token::TokenType::NEWLINE,
			TokenPosition::new(Position::new(4, 0), Position::new(5, 0)),
		),
		Token::new(
			gl_core::token::TokenType::EOF,
			TokenPosition::new(Position::new(2, 1), Position::new(2, 1)),
		),
	];
	let rtokens: Result<Vec<Token>, AnyError> = lexer.run(source, &module, &mut program);

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_identifier() {
	let mut lexer: Lexer = Lexer::new();
	let source: String = format!("identifier");
	let module: String = format!("tests/lexer/identifier");
	let mut program: ProgramState = ProgramState::new();

	let expected_tokens: Vec<Token> = vec![
		Token::new(
			gl_core::token::TokenType::IDENTIFIER(format!("identifier")),
			TokenPosition::new(Position::new(0, 0), Position::new(10, 0)),
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

#[test]
fn run_integer() {
	let mut lexer: Lexer = Lexer::new();
	let source: String = format!("1234567890");
	let module: String = format!("tests/lexer/integer");
	let mut program: ProgramState = ProgramState::new();

	let expected_tokens: Vec<Token> = vec![
		Token::new(
			gl_core::token::TokenType::INTEGER(format!("1234567890")),
			TokenPosition::new(Position::new(0, 0), Position::new(10, 0)),
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

#[test]
fn run_boolean() {
	let mut lexer: Lexer = Lexer::new();
	let source: String = format!("true false");
	let module: String = format!("tests/lexer/boolean");
	let mut program: ProgramState = ProgramState::new();

	let expected_tokens: Vec<Token> = vec![
		Token::new(
			gl_core::token::TokenType::BOOLEAN(true),
			TokenPosition::new(Position::new(0, 0), Position::new(4, 0)),
		),
		Token::new(
			gl_core::token::TokenType::BOOLEAN(false),
			TokenPosition::new(Position::new(5, 0), Position::new(10, 0)),
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

#[test]
fn run_string() {
	let mut lexer: Lexer = Lexer::new();
	let source: String = format!("\"text\"");
	let module: String = format!("tests/lexer/string");
	let mut program: ProgramState = ProgramState::new();

	let expected_tokens: Vec<Token> = vec![
		Token::new(
			gl_core::token::TokenType::STRING(format!("text")),
			TokenPosition::new(Position::new(0, 0), Position::new(6, 0)),
		),
		Token::new(
			gl_core::token::TokenType::EOF,
			TokenPosition::new(Position::new(6, 0), Position::new(6, 0)),
		),
	];
	let rtokens: Result<Vec<Token>, AnyError> = lexer.run(source, &module, &mut program);

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_operators() {
	let mut lexer: Lexer = Lexer::new();
	let source: String = format!("+-*/==!==!><>=<=");
	let module: String = format!("tests/lexer");
	let mut program: ProgramState = ProgramState::new();

	let expected_tokens: Vec<Token> = vec![
		Token::new(
			gl_core::token::TokenType::PLUS,
			TokenPosition::new(Position::new(0, 0), Position::new(1, 0)),
		),
		Token::new(
			gl_core::token::TokenType::MINUS,
			TokenPosition::new(Position::new(1, 0), Position::new(2, 0)),
		),
		Token::new(
			gl_core::token::TokenType::MULTIPLY,
			TokenPosition::new(Position::new(2, 0), Position::new(3, 0)),
		),
		Token::new(
			gl_core::token::TokenType::DIVIDE,
			TokenPosition::new(Position::new(3, 0), Position::new(4, 0)),
		),
		Token::new(
			gl_core::token::TokenType::EQUAL,
			TokenPosition::new(Position::new(4, 0), Position::new(6, 0)),
		),
		Token::new(
			gl_core::token::TokenType::NotEqual,
			TokenPosition::new(Position::new(6, 0), Position::new(8, 0)),
		),
		Token::new(
			gl_core::token::TokenType::ASSIGN,
			TokenPosition::new(Position::new(8, 0), Position::new(9, 0)),
		),
		Token::new(
			gl_core::token::TokenType::NOT,
			TokenPosition::new(Position::new(9, 0), Position::new(10, 0)),
		),
		Token::new(
			gl_core::token::TokenType::GreaterThan,
			TokenPosition::new(Position::new(10, 0), Position::new(11, 0)),
		),
		Token::new(
			gl_core::token::TokenType::LessThan,
			TokenPosition::new(Position::new(11, 0), Position::new(12, 0)),
		),
		Token::new(
			gl_core::token::TokenType::GreaterThanEqual,
			TokenPosition::new(Position::new(12, 0), Position::new(14, 0)),
		),
		Token::new(
			gl_core::token::TokenType::LessThanEqual,
			TokenPosition::new(Position::new(14, 0), Position::new(16, 0)),
		),
		Token::new(
			gl_core::token::TokenType::EOF,
			TokenPosition::new(Position::new(16, 0), Position::new(16, 0)),
		),
	];
	let rtokens: Result<Vec<Token>, AnyError> = lexer.run(source, &module, &mut program);

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_punctuations() {
	let mut lexer: Lexer = Lexer::new();
	let source: String = format!(".,;:()[]{{}}");
	let module: String = format!("tests/lexer");
	let mut program: ProgramState = ProgramState::new();

	let expected_tokens: Vec<Token> = vec![
		Token::new(
			gl_core::token::TokenType::DOT,
			TokenPosition::new(Position::new(0, 0), Position::new(1, 0)),
		),
		Token::new(
			gl_core::token::TokenType::COMMA,
			TokenPosition::new(Position::new(1, 0), Position::new(2, 0)),
		),
		Token::new(
			gl_core::token::TokenType::SEMICOLON,
			TokenPosition::new(Position::new(2, 0), Position::new(3, 0)),
		),
		Token::new(
			gl_core::token::TokenType::COLON,
			TokenPosition::new(Position::new(3, 0), Position::new(4, 0)),
		),
		Token::new(
			gl_core::token::TokenType::LParen,
			TokenPosition::new(Position::new(4, 0), Position::new(5, 0)),
		),
		Token::new(
			gl_core::token::TokenType::RParen,
			TokenPosition::new(Position::new(5, 0), Position::new(6, 0)),
		),
		Token::new(
			gl_core::token::TokenType::LBracket,
			TokenPosition::new(Position::new(6, 0), Position::new(7, 0)),
		),
		Token::new(
			gl_core::token::TokenType::RBracket,
			TokenPosition::new(Position::new(7, 0), Position::new(8, 0)),
		),
		Token::new(
			gl_core::token::TokenType::LBrace,
			TokenPosition::new(Position::new(8, 0), Position::new(9, 0)),
		),
		Token::new(
			gl_core::token::TokenType::RBrace,
			TokenPosition::new(Position::new(9, 0), Position::new(10, 0)),
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

#[test]
fn run_let() {
	let mut lexer: Lexer = Lexer::new();
	let source: String = format!("let");
	let module: String = format!("tests/lexer/let");
	let mut program: ProgramState = ProgramState::new();

	let expected_tokens: Vec<Token> = vec![
		Token::new(
			gl_core::token::TokenType::LET,
			TokenPosition::new(Position::new(0, 0), Position::new(3, 0)),
		),
		Token::new(
			gl_core::token::TokenType::EOF,
			TokenPosition::new(Position::new(3, 0), Position::new(3, 0)),
		),
	];
	let rtokens: Result<Vec<Token>, AnyError> = lexer.run(source, &module, &mut program);

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}
