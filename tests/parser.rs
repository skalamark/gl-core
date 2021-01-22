// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

extern crate gl_core;
extern crate num;

use gl_core::ast::{AbstractSyntaxTree, Expression, Literal, Statement};
use gl_core::error::ExceptionMain;
use gl_core::lexer::Lexer;
use gl_core::parser::Parser;
use gl_core::state::ProgramState;
use gl_core::token::Token;

#[test]
fn new() {
	let _parser: Parser = Parser::new();
}

#[test]
fn run_empty() {
	let mut lexer: Lexer = Lexer::new();
	let mut parser: Parser = Parser::new();
	let source: String = format!("");
	let module: String = format!("tests/parser/empty");
	let mut program: ProgramState = ProgramState::new();

	let rtokens: Result<Vec<Token>, ExceptionMain> = lexer.run(source, &module, &mut program);
	assert_eq!(false, rtokens.is_err());
	let tokens: Vec<Token> = rtokens.unwrap();

	let expected_ast: AbstractSyntaxTree = AbstractSyntaxTree::new();
	let rast: Result<AbstractSyntaxTree, ExceptionMain> = parser.run(tokens, &module, &mut program);

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap())
}

#[test]
fn run_null() {
	let mut lexer: Lexer = Lexer::new();
	let mut parser: Parser = Parser::new();
	let source: String = format!("null");
	let module: String = format!("tests/parser/null");
	let mut program: ProgramState = ProgramState::new();

	let rtokens: Result<Vec<Token>, ExceptionMain> = lexer.run(source, &module, &mut program);
	assert_eq!(false, rtokens.is_err());
	let tokens: Vec<Token> = rtokens.unwrap();

	let expected_ast: AbstractSyntaxTree = {
		let mut ast: AbstractSyntaxTree = AbstractSyntaxTree::new();

		ast.push(Statement::Expression(Expression::Literal(Literal::Null)));

		ast
	};
	let rast: Result<AbstractSyntaxTree, ExceptionMain> = parser.run(tokens, &module, &mut program);

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap())
}

#[test]
fn run_integer() {
	let mut lexer: Lexer = Lexer::new();
	let mut parser: Parser = Parser::new();
	let source: String = format!("1234567890");
	let module: String = format!("tests/parser/integer");
	let mut program: ProgramState = ProgramState::new();

	let rtokens: Result<Vec<Token>, ExceptionMain> = lexer.run(source, &module, &mut program);
	assert_eq!(false, rtokens.is_err());
	let tokens: Vec<Token> = rtokens.unwrap();

	let expected_ast: AbstractSyntaxTree = {
		let mut ast: AbstractSyntaxTree = AbstractSyntaxTree::new();

		ast.push(Statement::Expression(Expression::Literal(
			Literal::Integer(num::BigInt::parse_bytes(b"1234567890", 10).unwrap()),
		)));

		ast
	};
	let rast: Result<AbstractSyntaxTree, ExceptionMain> = parser.run(tokens, &module, &mut program);

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap())
}

#[test]
fn run_boolean() {
	let mut lexer: Lexer = Lexer::new();
	let mut parser: Parser = Parser::new();
	let source: String = format!("true;false");
	let module: String = format!("tests/parser/boolean");
	let mut program: ProgramState = ProgramState::new();

	let rtokens: Result<Vec<Token>, ExceptionMain> = lexer.run(source, &module, &mut program);
	assert_eq!(false, rtokens.is_err());
	let tokens: Vec<Token> = rtokens.unwrap();

	let expected_ast: AbstractSyntaxTree = {
		let mut ast: AbstractSyntaxTree = AbstractSyntaxTree::new();

		ast.push(Statement::Expression(Expression::Literal(
			Literal::Boolean(true),
		)));
		ast.push(Statement::Expression(Expression::Literal(
			Literal::Boolean(false),
		)));

		ast
	};
	let rast: Result<AbstractSyntaxTree, ExceptionMain> = parser.run(tokens, &module, &mut program);

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap())
}

#[test]
fn run_string() {
	let mut lexer: Lexer = Lexer::new();
	let mut parser: Parser = Parser::new();
	let source: String = format!("\"text\"");
	let module: String = format!("tests/parser/string");
	let mut program: ProgramState = ProgramState::new();

	let rtokens: Result<Vec<Token>, ExceptionMain> = lexer.run(source, &module, &mut program);
	assert_eq!(false, rtokens.is_err());
	let tokens: Vec<Token> = rtokens.unwrap();

	let expected_ast: AbstractSyntaxTree = {
		let mut ast: AbstractSyntaxTree = AbstractSyntaxTree::new();

		ast.push(Statement::Expression(Expression::Literal(Literal::String(
			format!("text"),
		))));

		ast
	};
	let rast: Result<AbstractSyntaxTree, ExceptionMain> = parser.run(tokens, &module, &mut program);

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap())
}

#[test]
fn run_vec() {
	let mut lexer: Lexer = Lexer::new();
	let mut parser: Parser = Parser::new();
	let source: String = format!("[42, \"Hello World\"]");
	let module: String = format!("tests/parser/vec");
	let mut program: ProgramState = ProgramState::new();

	let rtokens: Result<Vec<Token>, ExceptionMain> = lexer.run(source, &module, &mut program);
	assert_eq!(false, rtokens.is_err());
	let tokens: Vec<Token> = rtokens.unwrap();

	let expected_ast: AbstractSyntaxTree = {
		let mut ast: AbstractSyntaxTree = AbstractSyntaxTree::new();

		ast.push(Statement::Expression(Expression::Literal(Literal::Vec(
			vec![
				Expression::Literal(Literal::Integer(
					num::BigInt::parse_bytes(b"42", 10).unwrap(),
				)),
				Expression::Literal(Literal::String(format!("Hello World"))),
			],
		))));

		ast
	};
	let rast: Result<AbstractSyntaxTree, ExceptionMain> = parser.run(tokens, &module, &mut program);

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap())
}

#[test]
fn run_hashmap() {
	let mut lexer: Lexer = Lexer::new();
	let mut parser: Parser = Parser::new();
	let source: String = format!("{{\"name\": \"José\", \"age\": 17}}");
	let module: String = format!("tests/parser/hashmap");
	let mut program: ProgramState = ProgramState::new();

	let rtokens: Result<Vec<Token>, ExceptionMain> = lexer.run(source, &module, &mut program);
	assert_eq!(false, rtokens.is_err());
	let tokens: Vec<Token> = rtokens.unwrap();

	let expected_ast: AbstractSyntaxTree = {
		let mut ast: AbstractSyntaxTree = AbstractSyntaxTree::new();

		ast.push(Statement::Expression(Expression::Literal(
			Literal::HashMap(vec![
				(
					Expression::Literal(Literal::String(format!("name"))),
					Expression::Literal(Literal::String(format!("José"))),
				),
				(
					Expression::Literal(Literal::String(format!("age"))),
					Expression::Literal(Literal::Integer(
						num::BigInt::parse_bytes(b"17", 10).unwrap(),
					)),
				),
			]),
		)));

		ast
	};
	let rast: Result<AbstractSyntaxTree, ExceptionMain> = parser.run(tokens, &module, &mut program);

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap());
}

#[test]
fn run_let() {
	let mut lexer: Lexer = Lexer::new();
	let mut parser: Parser = Parser::new();
	let source: String = format!("let universo = 42");
	let module: String = format!("tests/parser/let");
	let mut program: ProgramState = ProgramState::new();

	let rtokens: Result<Vec<Token>, ExceptionMain> = lexer.run(source, &module, &mut program);
	assert_eq!(false, rtokens.is_err());
	let tokens: Vec<Token> = rtokens.unwrap();

	let expected_ast: AbstractSyntaxTree = {
		let mut ast: AbstractSyntaxTree = AbstractSyntaxTree::new();

		ast.push(Statement::Let(
			format!("universo"),
			Expression::Literal(Literal::Integer(
				num::BigInt::parse_bytes(b"42", 10).unwrap(),
			)),
		));

		ast
	};
	let rast: Result<AbstractSyntaxTree, ExceptionMain> = parser.run(tokens, &module, &mut program);

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap())
}
