// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AbstractSyntaxTree {
	pub statements: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block(pub Vec<Statement>);

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
	Let(String, Expression),
	Expression(Expression),
	ExpressionReturn(Expression),
	Fn {
		name: String,
		params: Vec<String>,
		body: Block,
	},
	Import(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
	Identifier(String),
	Literal(Literal),
	Prefix(Prefix, Box<Expression>),
	Infix(Infix, Box<Expression>, Box<Expression>),
	Fn {
		params: Vec<String>,
		body: Block,
	},
	Call {
		function: Box<Expression>,
		arguments: Vec<Expression>,
	},
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
	Null,
	Integer(BigInt),
	Float(BigRational),
	Boolean(bool),
	String(String),
	Vec(Vec<Expression>),
	HashMap(Vec<(Expression, Expression)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Prefix {
	Plus,  // +
	Minus, // -
	Not,   // !
}

#[derive(Clone, Debug, PartialEq)]
pub enum Infix {
	Plus,             // +
	Minus,            // -
	Multiply,         // *
	Divide,           // /
	Equal,            // ==
	NotEqual,         // !=
	GreaterThanEqual, // >=
	GreaterThan,      // >
	LessThanEqual,    // <=
	LessThan,         // <
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Precedence {
	Lowest,
	Equals,      // == or !=
	LessGreater, // > or < or >= or <=
	Sum,         // + or -
	Product,     // * or /
	Prefix,      // +X or -X or !X
	Call,        // function(x)
	Index,       // vec[index] or hashmap[key]
}

impl std::fmt::Display for Prefix {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Prefix::Plus => write!(f, "+"),
			Prefix::Minus => write!(f, "-"),
			Prefix::Not => write!(f, "!"),
		}
	}
}

impl std::fmt::Display for Infix {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Infix::Plus => write!(f, "+"),
			Infix::Minus => write!(f, "-"),
			Infix::Divide => write!(f, "/"),
			Infix::Multiply => write!(f, "*"),
			Infix::Equal => write!(f, "=="),
			Infix::NotEqual => write!(f, "!="),
			Infix::GreaterThanEqual => write!(f, ">="),
			Infix::GreaterThan => write!(f, ">"),
			Infix::LessThanEqual => write!(f, "<="),
			Infix::LessThan => write!(f, "<"),
		}
	}
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

impl Precedence {
	pub fn from_token(token: &Token) -> Self {
		Self::from_token_type(&token.typer)
	}

	pub fn from_token_type(token_type: &TokenType) -> Self {
		match token_type {
			TokenType::EQUAL | TokenType::NotEqual => Precedence::Equals,
			TokenType::LessThan | TokenType::LessThanEqual => Precedence::LessGreater,
			TokenType::GreaterThan | TokenType::GreaterThanEqual => Precedence::LessGreater,
			TokenType::PLUS | TokenType::MINUS => Precedence::Sum,
			TokenType::ASTERISK | TokenType::SLASH => Precedence::Product,
			TokenType::LeftBracket => Precedence::Index,
			TokenType::LeftParen => Precedence::Call,
			_ => Precedence::Lowest,
		}
	}
}
