// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use num::BigInt;

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
	Integer(BigInt),
}

impl AbstractSyntaxTree {
	pub fn new() -> Self {
		Self {
			statements: Vec::new(),
		}
	}

	pub fn push(&mut self, statement: Statement) {
		self.statements.push(statement);
	}
}
