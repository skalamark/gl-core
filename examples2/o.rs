// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

extern crate gl_core;

use gl_core::lexer::Lexer;
use gl_core::parser::Parser;

fn main() {
	let lexer: Lexer = Lexer::new_with_source(format!("989"));
	let mut parser: Parser = Parser::new_from_lexer(lexer);
	let module: String = format!("examples/lexer");

	match parser.next(&module) {
		Ok(a) => {
			println!("{:#?}", if a.is_some() { a.unwrap() } else { return })
		}
		Err(e) => {
			println!("{:#?}", e)
		}
	}
}
