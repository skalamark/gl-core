// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

extern crate gl_core;

use gl_core::ast::AbstractSyntaxTree;
use gl_core::error::Exception;
use gl_core::lexer::Lexer;
use gl_core::parser::Parser;
use gl_core::source::Source;

fn main() {
	let source: Source = Source::from_string(format!("42")).unwrap();
	let module: String = format!("examples/lexer");
	let lexer: Lexer = Lexer::new(source, &module);
	let mut parser: Parser = Parser::new(lexer);

	let rast: Result<AbstractSyntaxTree, Exception> = parser.run();
	assert_eq!(false, rast.is_err());

	let _ast: AbstractSyntaxTree = rast.unwrap();
}
