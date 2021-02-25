// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

extern crate gl_core;

use gl_core::preludes::*;

#[test]
fn new() {
	let _lexer: Lexer = Lexer::new(
		Source::from_string(format!("")).unwrap(),
		&format!("tests/lexer/new"),
	);
}

#[test]
fn run_empty() {
	let source: Source = Source::from_string(format!("")).unwrap();
	let module: String = format!("tests/lexer/empty");
	let mut lexer: Lexer = Lexer::new(source, &module);

	let expected_tokens: Vec<Token> = vec![Token::new(
		TokenType::EOF,
		TokenPosition::new(Position::new(0, 0), Position::new(0, 0)),
	)];
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_whitespaces() {
	let source: Source = Source::from_string(format!("  	\r\n\t ")).unwrap();
	let module: String = format!("tests/lexer/whitespaces");
	let mut lexer: Lexer = Lexer::new(source, &module);

	let expected_tokens: Vec<Token> = vec![
		Token::new(
			TokenType::NEWLINE,
			TokenPosition::new(Position::new(4, 0), Position::new(5, 0)),
		),
		Token::new(
			TokenType::EOF,
			TokenPosition::new(Position::new(2, 1), Position::new(2, 1)),
		),
	];
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_identifier() {
	let source: Source = Source::from_string(format!("identifier")).unwrap();
	let module: String = format!("tests/lexer/identifier");
	let mut lexer: Lexer = Lexer::new(source, &module);

	let expected_tokens: Vec<Token> = vec![
		Token::new(
			TokenType::IDENTIFIER(format!("identifier")),
			TokenPosition::new(Position::new(0, 0), Position::new(10, 0)),
		),
		Token::new(
			TokenType::EOF,
			TokenPosition::new(Position::new(10, 0), Position::new(10, 0)),
		),
	];
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_integer() {
	let source: Source = Source::from_string(format!("1234567890")).unwrap();
	let module: String = format!("tests/lexer/integer");
	let mut lexer: Lexer = Lexer::new(source, &module);

	let expected_tokens: Vec<Token> = vec![
		Token::new(
			TokenType::INTEGER(format!("1234567890")),
			TokenPosition::new(Position::new(0, 0), Position::new(10, 0)),
		),
		Token::new(
			TokenType::EOF,
			TokenPosition::new(Position::new(10, 0), Position::new(10, 0)),
		),
	];
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_float() {
	let source: Source = Source::from_string(format!("12345.67890")).unwrap();
	let module: String = format!("tests/lexer/integer");
	let mut lexer: Lexer = Lexer::new(source, &module);

	let expected_tokens: Vec<Token> = vec![
		Token::new(
			TokenType::FLOAT(format!("12345.67890")),
			TokenPosition::new(Position::new(0, 0), Position::new(11, 0)),
		),
		Token::new(
			TokenType::EOF,
			TokenPosition::new(Position::new(11, 0), Position::new(11, 0)),
		),
	];
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_boolean() {
	let source: Source = Source::from_string(format!("true false")).unwrap();
	let module: String = format!("tests/lexer/boolean");
	let mut lexer: Lexer = Lexer::new(source, &module);

	let expected_tokens: Vec<Token> = vec![
		Token::new(
			TokenType::BOOLEAN(true),
			TokenPosition::new(Position::new(0, 0), Position::new(4, 0)),
		),
		Token::new(
			TokenType::BOOLEAN(false),
			TokenPosition::new(Position::new(5, 0), Position::new(10, 0)),
		),
		Token::new(
			TokenType::EOF,
			TokenPosition::new(Position::new(10, 0), Position::new(10, 0)),
		),
	];
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_string() {
	let source: Source = Source::from_string(format!("\"text\"")).unwrap();
	let module: String = format!("tests/lexer/string");
	let mut lexer: Lexer = Lexer::new(source, &module);

	let expected_tokens: Vec<Token> = vec![
		Token::new(
			TokenType::STRING(format!("text")),
			TokenPosition::new(Position::new(0, 0), Position::new(6, 0)),
		),
		Token::new(
			TokenType::EOF,
			TokenPosition::new(Position::new(6, 0), Position::new(6, 0)),
		),
	];
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_vec() {
	let source: Source = Source::from_string(format!("[42, \"Hello World\"]")).unwrap();
	let module: String = format!("tests/lexer/vec");
	let mut lexer: Lexer = Lexer::new(source, &module);

	let expected_tokens: Vec<Token> = vec![
		Token::new(
			TokenType::LeftBracket,
			TokenPosition::new(Position::new(0, 0), Position::new(1, 0)),
		),
		Token::new(
			TokenType::INTEGER(format!("42")),
			TokenPosition::new(Position::new(1, 0), Position::new(3, 0)),
		),
		Token::new(
			TokenType::COMMA,
			TokenPosition::new(Position::new(3, 0), Position::new(4, 0)),
		),
		Token::new(
			TokenType::STRING(format!("Hello World")),
			TokenPosition::new(Position::new(5, 0), Position::new(18, 0)),
		),
		Token::new(
			TokenType::RightBracket,
			TokenPosition::new(Position::new(18, 0), Position::new(19, 0)),
		),
		Token::new(
			TokenType::EOF,
			TokenPosition::new(Position::new(19, 0), Position::new(19, 0)),
		),
	];
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_hashmap() {
	let source: Source =
		Source::from_string(format!("{{\"name\": \"José\", \"age\": 17}}")).unwrap();
	let module: String = format!("tests/lexer/hashmap");
	let mut lexer: Lexer = Lexer::new(source, &module);

	let expected_tokens: Vec<Token> = vec![
		Token::new(
			TokenType::LeftBrace,
			TokenPosition::new(Position::new(0, 0), Position::new(1, 0)),
		),
		Token::new(
			TokenType::STRING(format!("name")),
			TokenPosition::new(Position::new(1, 0), Position::new(7, 0)),
		),
		Token::new(
			TokenType::COLON,
			TokenPosition::new(Position::new(7, 0), Position::new(8, 0)),
		),
		Token::new(
			TokenType::STRING(format!("José")),
			TokenPosition::new(Position::new(9, 0), Position::new(15, 0)),
		),
		Token::new(
			TokenType::COMMA,
			TokenPosition::new(Position::new(15, 0), Position::new(16, 0)),
		),
		Token::new(
			TokenType::STRING(format!("age")),
			TokenPosition::new(Position::new(17, 0), Position::new(22, 0)),
		),
		Token::new(
			TokenType::COLON,
			TokenPosition::new(Position::new(22, 0), Position::new(23, 0)),
		),
		Token::new(
			TokenType::INTEGER(format!("17")),
			TokenPosition::new(Position::new(24, 0), Position::new(26, 0)),
		),
		Token::new(
			TokenType::RightBrace,
			TokenPosition::new(Position::new(26, 0), Position::new(27, 0)),
		),
		Token::new(
			TokenType::EOF,
			TokenPosition::new(Position::new(27, 0), Position::new(27, 0)),
		),
	];
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_operators() {
	let source: Source = Source::from_string(format!("+-*/==!==!><>=<=")).unwrap();
	let module: String = format!("tests/lexer");
	let mut lexer: Lexer = Lexer::new(source, &module);

	let expected_tokens: Vec<Token> = vec![
		Token::new(
			TokenType::PLUS,
			TokenPosition::new(Position::new(0, 0), Position::new(1, 0)),
		),
		Token::new(
			TokenType::MINUS,
			TokenPosition::new(Position::new(1, 0), Position::new(2, 0)),
		),
		Token::new(
			TokenType::ASTERISK,
			TokenPosition::new(Position::new(2, 0), Position::new(3, 0)),
		),
		Token::new(
			TokenType::SLASH,
			TokenPosition::new(Position::new(3, 0), Position::new(4, 0)),
		),
		Token::new(
			TokenType::EQUAL,
			TokenPosition::new(Position::new(4, 0), Position::new(6, 0)),
		),
		Token::new(
			TokenType::NotEqual,
			TokenPosition::new(Position::new(6, 0), Position::new(8, 0)),
		),
		Token::new(
			TokenType::ASSIGN,
			TokenPosition::new(Position::new(8, 0), Position::new(9, 0)),
		),
		Token::new(
			TokenType::BANG,
			TokenPosition::new(Position::new(9, 0), Position::new(10, 0)),
		),
		Token::new(
			TokenType::GreaterThan,
			TokenPosition::new(Position::new(10, 0), Position::new(11, 0)),
		),
		Token::new(
			TokenType::LessThan,
			TokenPosition::new(Position::new(11, 0), Position::new(12, 0)),
		),
		Token::new(
			TokenType::GreaterThanEqual,
			TokenPosition::new(Position::new(12, 0), Position::new(14, 0)),
		),
		Token::new(
			TokenType::LessThanEqual,
			TokenPosition::new(Position::new(14, 0), Position::new(16, 0)),
		),
		Token::new(
			TokenType::EOF,
			TokenPosition::new(Position::new(16, 0), Position::new(16, 0)),
		),
	];
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_punctuations() {
	let source: Source = Source::from_string(format!(".,;:()[]{{}}")).unwrap();
	let module: String = format!("tests/lexer");
	let mut lexer: Lexer = Lexer::new(source, &module);

	let expected_tokens: Vec<Token> = vec![
		Token::new(
			TokenType::DOT,
			TokenPosition::new(Position::new(0, 0), Position::new(1, 0)),
		),
		Token::new(
			TokenType::COMMA,
			TokenPosition::new(Position::new(1, 0), Position::new(2, 0)),
		),
		Token::new(
			TokenType::SEMICOLON,
			TokenPosition::new(Position::new(2, 0), Position::new(3, 0)),
		),
		Token::new(
			TokenType::COLON,
			TokenPosition::new(Position::new(3, 0), Position::new(4, 0)),
		),
		Token::new(
			TokenType::LeftParen,
			TokenPosition::new(Position::new(4, 0), Position::new(5, 0)),
		),
		Token::new(
			TokenType::RightParen,
			TokenPosition::new(Position::new(5, 0), Position::new(6, 0)),
		),
		Token::new(
			TokenType::LeftBracket,
			TokenPosition::new(Position::new(6, 0), Position::new(7, 0)),
		),
		Token::new(
			TokenType::RightBracket,
			TokenPosition::new(Position::new(7, 0), Position::new(8, 0)),
		),
		Token::new(
			TokenType::LeftBrace,
			TokenPosition::new(Position::new(8, 0), Position::new(9, 0)),
		),
		Token::new(
			TokenType::RightBrace,
			TokenPosition::new(Position::new(9, 0), Position::new(10, 0)),
		),
		Token::new(
			TokenType::EOF,
			TokenPosition::new(Position::new(10, 0), Position::new(10, 0)),
		),
	];
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_let() {
	let source: Source = Source::from_string(format!("let")).unwrap();
	let module: String = format!("tests/lexer/let");
	let mut lexer: Lexer = Lexer::new(source, &module);

	let expected_tokens: Vec<Token> = vec![
		Token::new(
			TokenType::LET,
			TokenPosition::new(Position::new(0, 0), Position::new(3, 0)),
		),
		Token::new(
			TokenType::EOF,
			TokenPosition::new(Position::new(3, 0), Position::new(3, 0)),
		),
	];
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}
