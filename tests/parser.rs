// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

extern crate gl_core;

use gl_core::preludes::*;

fn vec_statements2ast(statements: Vec<Statement>) -> AbstractSyntaxTree {
	let mut result: AbstractSyntaxTree = AbstractSyntaxTree::new();

	for statement in statements {
		result.push(statement);
	}

	result
}

#[test]
fn new() {
	let source: Source = Source::from_string("");
	let lexer: Lexer = Lexer::new(source, "tests/parser/new");
	let rparser: Result<Parser, Exception> = Parser::new(lexer);
	assert_eq!(false, rparser.is_err());
}

#[test]
fn run_empty() {
	let source: Source = Source::from_string("");
	let module: &str = "tests/parser/empty";
	let lexer: Lexer = Lexer::new(source, module);
	let rparser: Result<Parser, Exception> = Parser::new(lexer);
	assert_eq!(false, rparser.is_err());

	let expected_ast: AbstractSyntaxTree = AbstractSyntaxTree::new();
	let rast: Result<AbstractSyntaxTree, Exception> = rparser.unwrap().run();

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap());
}

#[test]
fn run_null() {
	let source: Source = Source::from_string("null");
	let module: &str = "tests/parser/null";
	let lexer: Lexer = Lexer::new(source, module);
	let rparser: Result<Parser, Exception> = Parser::new(lexer);
	assert_eq!(false, rparser.is_err());

	let expected_ast: AbstractSyntaxTree =
		vec_statements2ast(vec![Statement::ExpressionReturn(Literal::Null.into())]);
	let rast: Result<AbstractSyntaxTree, Exception> = rparser.unwrap().run();

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap());
}

#[test]
fn run_integer() {
	let source: Source = Source::from_string("1234567890");
	let module: &str = "tests/parser/integer";
	let lexer: Lexer = Lexer::new(source, module);
	let rparser: Result<Parser, Exception> = Parser::new(lexer);
	assert_eq!(false, rparser.is_err());

	let expected_ast: AbstractSyntaxTree = vec_statements2ast(vec![Statement::ExpressionReturn(
		BigInt::parse_bytes(b"1234567890", 10).unwrap().into(),
	)]);
	let rast: Result<AbstractSyntaxTree, Exception> = rparser.unwrap().run();

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap());
}

#[test]
fn run_float() {
	let source: Source = Source::from_string("12345.67890");
	let module: &str = "tests/parser/integer";
	let lexer: Lexer = Lexer::new(source, module);
	let rparser: Result<Parser, Exception> = Parser::new(lexer);
	assert_eq!(false, rparser.is_err());

	let expected_ast: AbstractSyntaxTree = vec_statements2ast(vec![Statement::ExpressionReturn(
		str_to_big_rational("12345.67890").unwrap().into(),
	)]);
	let rast: Result<AbstractSyntaxTree, Exception> = rparser.unwrap().run();

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap());
}

#[test]
fn run_boolean() {
	let source: Source = Source::from_string("true;false");
	let module: &str = "tests/parser/boolean";
	let lexer: Lexer = Lexer::new(source, module);
	let rparser: Result<Parser, Exception> = Parser::new(lexer);
	assert_eq!(false, rparser.is_err());

	let expected_ast: AbstractSyntaxTree = vec_statements2ast(vec![
		Statement::Expression(true.into()),
		Statement::ExpressionReturn(false.into()),
	]);
	let rast: Result<AbstractSyntaxTree, Exception> = rparser.unwrap().run();

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap());
}

#[test]
fn run_string() {
	let source: Source = Source::from_string("\"text\"");
	let module: &str = "tests/parser/string";
	let lexer: Lexer = Lexer::new(source, module);
	let rparser: Result<Parser, Exception> = Parser::new(lexer);
	assert_eq!(false, rparser.is_err());

	let expected_ast: AbstractSyntaxTree =
		vec_statements2ast(vec![Statement::ExpressionReturn(format!("text").into())]);
	let rast: Result<AbstractSyntaxTree, Exception> = rparser.unwrap().run();

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap());
}

#[test]
fn run_vec() {
	let source: Source = Source::from_string("[\"text\", 42, null]");
	let module: &str = "tests/parser/vec";
	let lexer: Lexer = Lexer::new(source, module);
	let rparser: Result<Parser, Exception> = Parser::new(lexer);
	assert_eq!(false, rparser.is_err());

	let expected_ast: AbstractSyntaxTree = vec_statements2ast(vec![Statement::ExpressionReturn(
		Expression::Literal(Literal::Vec(vec![
			Expression::Literal(format!("text").into()),
			42.to_bigint().unwrap().into(),
			Expression::Literal(Literal::Null),
		])),
	)]);
	let rast: Result<AbstractSyntaxTree, Exception> = rparser.unwrap().run();

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap());
}

#[test]
fn run_tuple() {
	let source: Source = Source::from_string("(\"text\", 42, null)");
	let module: &str = "tests/parser/vec";
	let lexer: Lexer = Lexer::new(source, module);
	let rparser: Result<Parser, Exception> = Parser::new(lexer);
	assert_eq!(false, rparser.is_err());

	let expected_ast: AbstractSyntaxTree = vec_statements2ast(vec![Statement::ExpressionReturn(
		Expression::Literal(Literal::Tuple(vec![
			Expression::Literal(format!("text").into()),
			42.to_bigint().unwrap().into(),
			Expression::Literal(Literal::Null),
		])),
	)]);
	let rast: Result<AbstractSyntaxTree, Exception> = rparser.unwrap().run();

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap());
}

#[test]
fn run_hashmap() {
	let source: Source = Source::from_string("{\"text\": \"Hello\", 42: \"age\"}");
	let module: &str = "tests/parser/hashmap";
	let lexer: Lexer = Lexer::new(source, module);
	let rparser: Result<Parser, Exception> = Parser::new(lexer);
	assert_eq!(false, rparser.is_err());

	let expected_ast: AbstractSyntaxTree = vec_statements2ast(vec![Statement::ExpressionReturn(
		Expression::Literal(Literal::HashMap(vec![
			(
				Expression::Literal(format!("text").into()),
				Expression::Literal(format!("Hello").into()),
			),
			(42.to_bigint().unwrap().into(), Expression::Literal(format!("age").into())),
		])),
	)]);
	let rast: Result<AbstractSyntaxTree, Exception> = rparser.unwrap().run();

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap());
}

#[test]
fn run_let() {
	let source: Source = Source::from_string("let universo = 42");
	let module: &str = "tests/parser/let";
	let lexer: Lexer = Lexer::new(source, module);
	let rparser: Result<Parser, Exception> = Parser::new(lexer);
	assert_eq!(false, rparser.is_err());

	let expected_ast: AbstractSyntaxTree = vec_statements2ast(vec![Statement::Let(
		format!("universo"),
		BigInt::parse_bytes(b"42", 10).unwrap().into(),
	)]);
	let rast: Result<AbstractSyntaxTree, Exception> = rparser.unwrap().run();

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap());
}

#[test]
fn run_fn_statement() {
	let source: Source = Source::from_string("fn main() {}");
	let module: &str = "tests/parser/fn_statement";
	let lexer: Lexer = Lexer::new(source, module);
	let rparser: Result<Parser, Exception> = Parser::new(lexer);
	assert_eq!(false, rparser.is_err());

	let expected_ast: AbstractSyntaxTree = vec_statements2ast(vec![Statement::Fn {
		name: String::from("main"),
		params: Vec::new(),
		body: Block(Vec::new()),
	}]);
	let rast: Result<AbstractSyntaxTree, Exception> = rparser.unwrap().run();

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap());
}

#[test]
fn run_fn_expression() {
	let source: Source = Source::from_string("fn () {}");
	let module: &str = "tests/parser/fn_expression";
	let lexer: Lexer = Lexer::new(source, module);
	let rparser: Result<Parser, Exception> = Parser::new(lexer);
	assert_eq!(false, rparser.is_err());

	let expected_ast: AbstractSyntaxTree =
		vec_statements2ast(vec![Statement::ExpressionReturn(Expression::Fn {
			params: Vec::new(),
			body: Block(Vec::new()),
		})]);
	let rast: Result<AbstractSyntaxTree, Exception> = rparser.unwrap().run();

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap());
}

#[test]
fn run_import() {
	let source: Source = Source::from_string("import \"mylib\"");
	let module: &str = "tests/parser/import";
	let lexer: Lexer = Lexer::new(source, module);
	let rparser: Result<Parser, Exception> = Parser::new(lexer);
	assert_eq!(false, rparser.is_err());

	let expected_ast: AbstractSyntaxTree =
		vec_statements2ast(vec![Statement::Import(String::from("mylib"))]);
	let rast: Result<AbstractSyntaxTree, Exception> = rparser.unwrap().run();

	assert_eq!(false, rast.is_err());
	assert_eq!(expected_ast, rast.unwrap());
}
