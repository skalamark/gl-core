// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

extern crate gl_core;

use gl_core::ast::AbstractSyntaxTree;
use gl_core::lexer::Lexer;
use gl_core::parser::Parser;
use gl_core::state::ProgramState;
use gl_core::token::Token;

fn main() {
	let mut lexer: Lexer = Lexer::new();
	let mut parser: Parser = Parser::new();
	let source: String = format!("42");
	let module: String = format!("examples/lexer");
	let mut program: ProgramState = ProgramState::new();

	let tokens: Vec<Token> = match lexer.run(source, &module, &mut program) {
		Ok(tokens) => tokens,
		Err(exception) => {
			println!("{}", exception);
			return;
		}
	};

	let _ast: AbstractSyntaxTree = match parser.run(tokens, &module, &mut program) {
		Ok(ast) => ast,
		Err(exception) => {
			println!("{}", exception);
			return;
		}
	};
}
