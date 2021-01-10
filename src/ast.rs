// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

pub struct AbstractSyntaxTree {
	pub statements: Vec<Statement>,
}

pub struct Block(Vec<Statement>);

pub enum Statement {
	Expression(Expression),
}

pub enum Expression {
	Null,
}

impl AbstractSyntaxTree {
	pub fn new() -> Self {
		Self {
			statements: Vec::new(),
		}
	}
}
