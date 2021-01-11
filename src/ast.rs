// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

#[derive(Debug, PartialEq)]
pub struct AbstractSyntaxTree {
	pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct Block(Vec<Statement>);

#[derive(Debug, PartialEq)]
pub enum Statement {
	Expression(Expression),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
	Null,
	Literal(Literal),
}

#[derive(Debug, PartialEq)]
pub enum Literal {
	Integer(i32),
}

impl AbstractSyntaxTree {
	pub fn new() -> Self {
		Self {
			statements: Vec::new(),
		}
	}
}
