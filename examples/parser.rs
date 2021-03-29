// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

extern crate gl_core;

use gl_core::preludes::*;

fn main() {
	let source: Source = Source::from_string("42");
	let module: &str = "examples/lexer";
	let lexer: Lexer = Lexer::new(source, module);
	let parser: Result<Parser, Exception> = Parser::new(lexer);

	let rast: Result<AbstractSyntaxTree, Exception> = parser.unwrap().run();
	assert_eq!(false, rast.is_err());

	let _ast: AbstractSyntaxTree = rast.unwrap();
}
