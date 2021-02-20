// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

extern crate gl_core;

use gl_core::preludes::*;

#[test]
fn new() {
	let source: Source = Source::new_from_string(format!("")).unwrap();
	let lexer: Lexer = Lexer::new(source, &format!("tests/lexer/new"));
	let _parser: Parser = Parser::new(lexer);
}

#[test]
fn run_empty() {
	let source: Source = Source::new_from_string(format!("")).unwrap();
	let module: String = format!("tests/parser/empty");
	let lexer: Lexer = Lexer::new(source, &module);
	let mut parser: Parser = Parser::new(lexer);

	let expected_ast: AbstractSyntaxTree = AbstractSyntaxTree::new();
	let rast: Result<AbstractSyntaxTree, Exception> = parser.run();

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap())
}

#[test]
fn run_null() {
	let source: Source = Source::new_from_string(format!("null")).unwrap();
	let module: String = format!("tests/parser/null");
	let lexer: Lexer = Lexer::new(source, &module);
	let mut parser: Parser = Parser::new(lexer);

	let expected_ast: AbstractSyntaxTree = {
		let mut ast: AbstractSyntaxTree = AbstractSyntaxTree::new();

		ast.push(Statement::ExpressionReturn(Expression::Literal(
			Literal::Null,
		)));

		ast
	};
	let rast: Result<AbstractSyntaxTree, Exception> = parser.run();

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap())
}

#[test]
fn run_integer() {
	let source: Source = Source::new_from_string(format!("1234567890")).unwrap();
	let module: String = format!("tests/parser/integer");
	let lexer: Lexer = Lexer::new(source, &module);
	let mut parser: Parser = Parser::new(lexer);

	let expected_ast: AbstractSyntaxTree = {
		let mut ast: AbstractSyntaxTree = AbstractSyntaxTree::new();

		ast.push(Statement::ExpressionReturn(Expression::Literal(
			Literal::Integer(num::BigInt::parse_bytes(b"1234567890", 10).unwrap()),
		)));

		ast
	};
	let rast: Result<AbstractSyntaxTree, Exception> = parser.run();

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap())
}

#[test]
fn run_float() {
	let source: Source = Source::new_from_string(format!("12345.67890")).unwrap();
	let module: String = format!("tests/parser/integer");
	let lexer: Lexer = Lexer::new(source, &module);
	let mut parser: Parser = Parser::new(lexer);

	let expected_ast: AbstractSyntaxTree = {
		let mut ast: AbstractSyntaxTree = AbstractSyntaxTree::new();

		ast.push(Statement::ExpressionReturn(Expression::Literal(
			Literal::Float(BigRational::new(
				12345.to_bigint().unwrap(),
				67890.to_bigint().unwrap(),
			)),
		)));

		ast
	};
	let rast: Result<AbstractSyntaxTree, Exception> = parser.run();

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap())
}

#[test]
fn run_boolean() {
	let source: Source = Source::new_from_string(format!("true;false")).unwrap();
	let module: String = format!("tests/parser/boolean");
	let lexer: Lexer = Lexer::new(source, &module);
	let mut parser: Parser = Parser::new(lexer);

	let expected_ast: AbstractSyntaxTree = {
		let mut ast: AbstractSyntaxTree = AbstractSyntaxTree::new();

		ast.push(Statement::Expression(Expression::Literal(
			Literal::Boolean(true),
		)));
		ast.push(Statement::ExpressionReturn(Expression::Literal(
			Literal::Boolean(false),
		)));

		ast
	};
	let rast: Result<AbstractSyntaxTree, Exception> = parser.run();

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap())
}

#[test]
fn run_string() {
	let source: Source = Source::new_from_string(format!("\"text\"")).unwrap();
	let module: String = format!("tests/parser/string");
	let lexer: Lexer = Lexer::new(source, &module);
	let mut parser: Parser = Parser::new(lexer);

	let expected_ast: AbstractSyntaxTree = {
		let mut ast: AbstractSyntaxTree = AbstractSyntaxTree::new();

		ast.push(Statement::ExpressionReturn(Expression::Literal(
			Literal::String(format!("text")),
		)));

		ast
	};
	let rast: Result<AbstractSyntaxTree, Exception> = parser.run();

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap())
}

#[test]
fn run_vec() {
	let source: Source = Source::new_from_string(format!("[42, \"Hello World\"]")).unwrap();
	let module: String = format!("tests/parser/vec");
	let lexer: Lexer = Lexer::new(source, &module);
	let mut parser: Parser = Parser::new(lexer);

	let expected_ast: AbstractSyntaxTree = {
		let mut ast: AbstractSyntaxTree = AbstractSyntaxTree::new();

		ast.push(Statement::ExpressionReturn(Expression::Literal(
			Literal::Vec(vec![
				Expression::Literal(Literal::Integer(
					num::BigInt::parse_bytes(b"42", 10).unwrap(),
				)),
				Expression::Literal(Literal::String(format!("Hello World"))),
			]),
		)));

		ast
	};
	let rast: Result<AbstractSyntaxTree, Exception> = parser.run();

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap())
}

#[test]
fn run_hashmap() {
	let source: Source =
		Source::new_from_string(format!("{{\"name\": \"José\", \"age\": 17}}")).unwrap();
	let module: String = format!("tests/parser/hashmap");
	let lexer: Lexer = Lexer::new(source, &module);
	let mut parser: Parser = Parser::new(lexer);

	let expected_ast: AbstractSyntaxTree = {
		let mut ast: AbstractSyntaxTree = AbstractSyntaxTree::new();

		ast.push(Statement::ExpressionReturn(Expression::Literal(
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
	let rast: Result<AbstractSyntaxTree, Exception> = parser.run();

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap());
}

#[test]
fn run_let() {
	let source: Source = Source::new_from_string(format!("let universo = 42")).unwrap();
	let module: String = format!("tests/parser/let");
	let lexer: Lexer = Lexer::new(source, &module);
	let mut parser: Parser = Parser::new(lexer);

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
	let rast: Result<AbstractSyntaxTree, Exception> = parser.run();

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap())
}
