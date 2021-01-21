// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use num::BigInt;

#[derive(Clone, Debug, PartialEq)]
pub struct AbstractSyntaxTree {
	pub statements: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block(Vec<Statement>);

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
	Let(String, Expression),
	Expression(Expression),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
	Identifier(String),
	Literal(Literal),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
	Null,
	Integer(BigInt),
	Boolean(bool),
	String(String),
	Vec(Vec<Expression>),
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
