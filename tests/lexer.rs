// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

extern crate gl_core;

use gl_core::preludes::*;
use TokenType::*;

fn vec_tokens_positions2vec_token(
	tokens_positions: Vec<(TokenType, (usize, usize), (usize, usize))>,
) -> Vec<Token> {
	let mut result: Vec<Token> = Vec::new();

	for (typer, pstart, pend) in tokens_positions {
		result.push(Token::new(
			typer,
			TokenPosition::new(Position::new(pstart.0, pstart.1), Position::new(pend.0, pend.1)),
		));
	}

	result
}

#[test]
fn new() { let _lexer: Lexer = Lexer::new(Source::from_string(""), "tests/lexer/new"); }

#[test]
fn run_empty() {
	let source: Source = Source::from_string("");
	let module: &str = "tests/lexer/empty";
	let mut lexer: Lexer = Lexer::new(source, module);

	let expected_tokens: Vec<Token> = vec![Token::default()];
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_whitespaces() {
	let source: Source = Source::from_string("  	\r\n\t ");
	let module: &str = "tests/lexer/whitespaces";
	let mut lexer: Lexer = Lexer::new(source, module);

	let expected_tokens: Vec<Token> =
		vec_tokens_positions2vec_token(vec![(NEWLINE, (4, 0), (5, 0)), (EOF, (2, 1), (2, 1))]);
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_identifier() {
	let source: Source = Source::from_string("identifier");
	let module: &str = "tests/lexer/identifier";
	let mut lexer: Lexer = Lexer::new(source, module);

	let expected_tokens: Vec<Token> = vec_tokens_positions2vec_token(vec![
		(IDENTIFIER(format!("identifier")), (0, 0), (10, 0)),
		(EOF, (10, 0), (10, 0)),
	]);
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_integer() {
	let source: Source = Source::from_string("1234567890");
	let module: &str = "tests/lexer/integer";
	let mut lexer: Lexer = Lexer::new(source, module);

	let expected_tokens: Vec<Token> = vec_tokens_positions2vec_token(vec![
		(INTEGER(format!("1234567890")), (0, 0), (10, 0)),
		(EOF, (10, 0), (10, 0)),
	]);
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_float() {
	let source: Source = Source::from_string("12345.67890");
	let module: &str = "tests/lexer/integer";
	let mut lexer: Lexer = Lexer::new(source, module);

	let expected_tokens: Vec<Token> = vec_tokens_positions2vec_token(vec![
		(FLOAT(format!("12345.67890")), (0, 0), (11, 0)),
		(EOF, (11, 0), (11, 0)),
	]);
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_boolean() {
	let source: Source = Source::from_string("true false");
	let module: &str = "tests/lexer/boolean";
	let mut lexer: Lexer = Lexer::new(source, module);

	let expected_tokens: Vec<Token> = vec_tokens_positions2vec_token(vec![
		(BOOLEAN(true), (0, 0), (4, 0)),
		(BOOLEAN(false), (5, 0), (10, 0)),
		(EOF, (10, 0), (10, 0)),
	]);
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_string() {
	let source: Source = Source::from_string("\"text\"");
	let module: &str = "tests/lexer/string";
	let mut lexer: Lexer = Lexer::new(source, module);

	let expected_tokens: Vec<Token> = vec_tokens_positions2vec_token(vec![
		(STRING(format!("text")), (0, 0), (6, 0)),
		(EOF, (6, 0), (6, 0)),
	]);
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_vec() {
	let source: Source = Source::from_string("[42, \"Hello World\"]");
	let module: &str = "tests/lexer/vec";
	let mut lexer: Lexer = Lexer::new(source, module);

	let expected_tokens: Vec<Token> = vec_tokens_positions2vec_token(vec![
		(LeftBracket, (0, 0), (1, 0)),
		(INTEGER(format!("42")), (1, 0), (3, 0)),
		(COMMA, (3, 0), (4, 0)),
		(STRING(format!("Hello World")), (5, 0), (18, 0)),
		(RightBracket, (18, 0), (19, 0)),
		(EOF, (19, 0), (19, 0)),
	]);
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_tuple() {
	let source: Source = Source::from_string("(42, \"Hello World\")");
	let module: &str = "tests/lexer/vec";
	let mut lexer: Lexer = Lexer::new(source, module);

	let expected_tokens: Vec<Token> = vec_tokens_positions2vec_token(vec![
		(LeftParen, (0, 0), (1, 0)),
		(INTEGER(format!("42")), (1, 0), (3, 0)),
		(COMMA, (3, 0), (4, 0)),
		(STRING(format!("Hello World")), (5, 0), (18, 0)),
		(RightParen, (18, 0), (19, 0)),
		(EOF, (19, 0), (19, 0)),
	]);
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_hashmap() {
	let source: Source = Source::from_string("{\"name\": \"José\", \"age\": 17}");
	let module: &str = "tests/lexer/hashmap";
	let mut lexer: Lexer = Lexer::new(source, module);

	let expected_tokens: Vec<Token> = vec_tokens_positions2vec_token(vec![
		(LeftBrace, (0, 0), (1, 0)),
		(STRING(format!("name")), (1, 0), (7, 0)),
		(COLON, (7, 0), (8, 0)),
		(STRING(format!("José")), (9, 0), (15, 0)),
		(COMMA, (15, 0), (16, 0)),
		(STRING(format!("age")), (17, 0), (22, 0)),
		(COLON, (22, 0), (23, 0)),
		(INTEGER(format!("17")), (24, 0), (26, 0)),
		(RightBrace, (26, 0), (27, 0)),
		(EOF, (27, 0), (27, 0)),
	]);
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_operators() {
	let source: Source = Source::from_string("+-*==/!==!><>=<=");
	let module: &str = "tests/lexer";
	let mut lexer: Lexer = Lexer::new(source, module);

	let expected_tokens: Vec<Token> = vec_tokens_positions2vec_token(vec![
		(PLUS, (0, 0), (1, 0)),
		(MINUS, (1, 0), (2, 0)),
		(ASTERISK, (2, 0), (3, 0)),
		(EQUAL, (3, 0), (5, 0)),
		(SLASH, (5, 0), (6, 0)),
		(NotEqual, (6, 0), (8, 0)),
		(ASSIGN, (8, 0), (9, 0)),
		(BANG, (9, 0), (10, 0)),
		(GreaterThan, (10, 0), (11, 0)),
		(LessThan, (11, 0), (12, 0)),
		(GreaterThanEqual, (12, 0), (14, 0)),
		(LessThanEqual, (14, 0), (16, 0)),
		(EOF, (16, 0), (16, 0)),
	]);
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_punctuations() {
	let source: Source = Source::from_string(".,;:()[]{}");
	let module: &str = "tests/lexer";
	let mut lexer: Lexer = Lexer::new(source, module);

	let expected_tokens: Vec<Token> = vec_tokens_positions2vec_token(vec![
		(DOT, (0, 0), (1, 0)),
		(COMMA, (1, 0), (2, 0)),
		(SEMICOLON, (2, 0), (3, 0)),
		(COLON, (3, 0), (4, 0)),
		(LeftParen, (4, 0), (5, 0)),
		(RightParen, (5, 0), (6, 0)),
		(LeftBracket, (6, 0), (7, 0)),
		(RightBracket, (7, 0), (8, 0)),
		(LeftBrace, (8, 0), (9, 0)),
		(RightBrace, (9, 0), (10, 0)),
		(EOF, (10, 0), (10, 0)),
	]);
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}

#[test]
fn run_keywords() {
	let source: Source = Source::from_string("let fn import");
	let module: &str = "tests/lexer/keywords";
	let mut lexer: Lexer = Lexer::new(source, module);

	let expected_tokens: Vec<Token> = vec_tokens_positions2vec_token(vec![
		(LET, (0, 0), (3, 0)),
		(FN, (4, 0), (6, 0)),
		(IMPORT, (7, 0), (13, 0)),
		(EOF, (13, 0), (13, 0)),
	]);
	let rtokens: Result<Vec<Token>, Exception> = lexer.run();

	assert_eq!(false, rtokens.is_err());
	assert_eq!(expected_tokens, rtokens.unwrap())
}
