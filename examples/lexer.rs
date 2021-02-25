// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

extern crate gl_core;

use gl_core::error::Exception;
use gl_core::lexer::Lexer;
use gl_core::source::Source;
use gl_core::token::Token;

fn main() {
	let source: Source = Source::from_string(format!("42")).unwrap();
	let module: String = format!("examples/lexer");
	let mut lexer: Lexer = Lexer::new(source, &module);

	let rtokens: Result<Vec<Token>, Exception> = lexer.run();
	assert_eq!(false, rtokens.is_err());

	let _tokens: Vec<Token> = rtokens.unwrap();
}
