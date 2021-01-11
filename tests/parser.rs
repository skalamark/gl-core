// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

extern crate gl_core;

use gl_core::ast::{AbstractSyntaxTree, Expression, Literal, Statement};
use gl_core::error::AnyError;
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
	let module: String = format!("crate");
	let mut program: ProgramState = ProgramState::new();

	let rtokens: Result<Vec<Token>, AnyError> = lexer.run(source, &module, &mut program);
	assert_eq!(false, rtokens.is_err());
	let tokens: Vec<Token> = rtokens.unwrap();

	let expected_ast: AbstractSyntaxTree = AbstractSyntaxTree::new();
	let rast: Result<AbstractSyntaxTree, AnyError> = parser.run(tokens, &module, &mut program);

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap())
}

#[test]
fn run_whitespaces() {
	let mut lexer: Lexer = Lexer::new();
	let mut parser: Parser = Parser::new();
	let source: String = format!("  	\r\n\t ");
	let module: String = format!("crate");
	let mut program: ProgramState = ProgramState::new();

	let rtokens: Result<Vec<Token>, AnyError> = lexer.run(source, &module, &mut program);
	assert_eq!(false, rtokens.is_err());
	let tokens: Vec<Token> = rtokens.unwrap();

	let expected_ast: AbstractSyntaxTree = AbstractSyntaxTree::new();
	let rast: Result<AbstractSyntaxTree, AnyError> = parser.run(tokens, &module, &mut program);

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap())
}

#[test]
fn run_digits() {
	let mut lexer: Lexer = Lexer::new();
	let mut parser: Parser = Parser::new();
	let source: String = format!("1234567890");
	let module: String = format!("crate");
	let mut program: ProgramState = ProgramState::new();

	let rtokens: Result<Vec<Token>, AnyError> = lexer.run(source, &module, &mut program);
	assert_eq!(false, rtokens.is_err());
	let tokens: Vec<Token> = rtokens.unwrap();

	let expected_ast: AbstractSyntaxTree = {
		let mut ast: AbstractSyntaxTree = AbstractSyntaxTree::new();

		ast.push(Statement::Expression(Expression::Literal(
			Literal::Integer(1234567890),
		)));

		ast
	};
	let rast: Result<AbstractSyntaxTree, AnyError> = parser.run(tokens, &module, &mut program);

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap())
}
