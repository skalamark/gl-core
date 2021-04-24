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
	Fn { name: String, params: Vec<String>, body: Block },
	Import(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
	Identifier(String),
	Literal(Literal),
	Prefix(Prefix, Box<Expression>),
	Infix(Infix, Box<Expression>, Box<Expression>),
	Fn { params: Vec<String>, body: Block },
	Call { function: Box<Expression>, arguments: Vec<Expression> },
	Index(Box<Expression>, Box<Expression>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
	Null,
	Integer(BigInt),
	Float(BigRational),
	Boolean(bool),
	String(String),
	Vec(Vec<Expression>),
	Tuple(Vec<Expression>),
	HashMap(Vec<(Expression, Expression)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Prefix {
	Not,   // !
	Plus,  // +
	Minus, // -
}

#[derive(Clone, Debug, PartialEq)]
pub enum Infix {
	Plus,             // +
	Minus,            // -
	Multiply,         // *
	Divide,           // /
	Equal,            // ==
	NotEqual,         // !=
	LessThanEqual,    // <=
	LessThan,         // <
	GreaterThanEqual, // >=
	GreaterThan,      // >
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Precedence {
	Lowest,
	Comma,          // ,
	Equality,       // == or !=
	Relational,     // > or < or >= or <=
	Additive,       // + or -
	Multiplicative, // / or *
	Call,           // function(x)
	Index,          // vec[index] or hashmap[key]
}

impl std::fmt::Display for Prefix {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Prefix::Not => write!(f, "!"),
			Prefix::Plus => write!(f, "+"),
			Prefix::Minus => write!(f, "-"),
		}
	}
}

impl std::fmt::Display for Infix {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Infix::Plus => write!(f, "+"),
			Infix::Minus => write!(f, "-"),
			Infix::Multiply => write!(f, "*"),
			Infix::Divide => write!(f, "/"),
			Infix::Equal => write!(f, "=="),
			Infix::NotEqual => write!(f, "!="),
			Infix::LessThanEqual => write!(f, "<="),
			Infix::LessThan => write!(f, "<"),
			Infix::GreaterThanEqual => write!(f, ">="),
			Infix::GreaterThan => write!(f, ">"),
		}
	}
}

impl AbstractSyntaxTree {
	pub fn new() -> Self { Self { statements: Vec::new() } }

	pub fn push(&mut self, statement: Statement) { self.statements.push(statement); }
}

impl Precedence {
	pub fn from_token(token: &Token) -> Self { Self::from_token_type(&token.typer) }

	pub fn from_token_type(token_type: &TokenType) -> Self {
		match token_type {
			TokenType::COMMA => Precedence::Comma,
			TokenType::EQUAL | TokenType::NotEqual => Precedence::Equality,
			TokenType::LessThan
			| TokenType::LessThanEqual
			| TokenType::GreaterThan
			| TokenType::GreaterThanEqual => Precedence::Relational,
			TokenType::PLUS | TokenType::MINUS => Precedence::Additive,
			TokenType::ASTERISK | TokenType::SLASH => Precedence::Multiplicative,
			TokenType::LeftParen => Precedence::Call,
			TokenType::LeftBracket => Precedence::Index,
			_ => Precedence::Lowest,
		}
	}
}

// Into Statement

impl Into<Statement> for Expression {
	fn into(self) -> Statement { Statement::Expression(self) }
}

impl Into<Statement> for Literal {
	fn into(self) -> Statement { Statement::Expression(self.into()) }
}

// Into Expression

impl Into<Expression> for Literal {
	fn into(self) -> Expression { Expression::Literal(self) }
}

impl Into<Expression> for BigInt {
	fn into(self) -> Expression { Expression::Literal(self.into()) }
}

impl Into<Expression> for BigRational {
	fn into(self) -> Expression { Expression::Literal(self.into()) }
}

impl Into<Expression> for bool {
	fn into(self) -> Expression { Expression::Literal(self.into()) }
}

impl Into<Expression> for String {
	fn into(self) -> Expression { Expression::Literal(self.into()) }
}

impl Into<Expression> for Vec<Expression> {
	fn into(self) -> Expression { Expression::Literal(self.into()) }
}

impl Into<Expression> for Vec<(Expression, Expression)> {
	fn into(self) -> Expression { Expression::Literal(self.into()) }
}

// Into Literal

impl Into<Literal> for BigInt {
	fn into(self) -> Literal { Literal::Integer(self) }
}

impl Into<Literal> for BigRational {
	fn into(self) -> Literal { Literal::Float(self) }
}

impl Into<Literal> for bool {
	fn into(self) -> Literal { Literal::Boolean(self) }
}

impl Into<Literal> for String {
	fn into(self) -> Literal { Literal::String(self) }
}

impl Into<Literal> for Vec<Expression> {
	fn into(self) -> Literal { Literal::Vec(self) }
}

impl Into<Literal> for Vec<(Expression, Expression)> {
	fn into(self) -> Literal { Literal::HashMap(self) }
}
