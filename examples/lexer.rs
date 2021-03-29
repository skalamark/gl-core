// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

extern crate gl_core;

use gl_core::preludes::*;

fn main() {
	let source: Source = Source::from_string("42");
	let module: &str = "examples/lexer";
	let mut lexer: Lexer = Lexer::new(source, module);

	let rtokens: Result<Vec<Token>, Exception> = lexer.run();
	assert_eq!(false, rtokens.is_err());

	let _tokens: Vec<Token> = rtokens.unwrap();
}
