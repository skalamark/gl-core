// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use core::cmp::PartialOrd;
use core::ops::{Add, Div, Mul, Sub};

use crate::preludes::*;

impl Add for Object {
	type Output = Result<Self, Exception>;

	fn add(self, other: Self) -> Result<Self, Exception> {
		match &self {
			Object::Integer(lhs) =>
				if let Object::Integer(rhs) = other {
					Ok(Object::Integer(lhs + rhs))
				} else if let Object::Float(rhs) = other {
					Ok(Object::Float(str_to_big_rational(&lhs.to_string()).unwrap() + rhs))
				} else if let Object::Boolean(rhs) = other {
					let rhs2bigint: BigInt = match rhs {
						true => 1.to_bigint().unwrap(),
						false => 0.to_bigint().unwrap(),
					};
					Ok(Object::Integer(lhs + rhs2bigint))
				} else {
					Err(unsupported_operand_type("+", self.typer(), other.typer()))
				},
			Object::Float(lhs) =>
				if let Object::Integer(rhs) = other {
					Ok(Object::Float(lhs + str_to_big_rational(&rhs.to_string()).unwrap()))
				} else if let Object::Float(rhs) = other {
					Ok(Object::Float(lhs + rhs))
				} else if let Object::Boolean(rhs) = other {
					let rhs2bigrational: BigRational = match rhs {
						true => str_to_big_rational("1").unwrap(),
						false => str_to_big_rational("0").unwrap(),
					};
					Ok(Object::Float(lhs + rhs2bigrational))
				} else {
					Err(unsupported_operand_type("+", self.typer(), other.typer()))
				},
			Object::Boolean(lhs) =>
				if let Object::Integer(rhs) = other {
					let lhs2bigint: BigInt = match lhs {
						true => 1.to_bigint().unwrap(),
						false => 0.to_bigint().unwrap(),
					};
					Ok(Object::Integer(lhs2bigint + rhs))
				} else if let Object::Float(rhs) = other {
					let lhs2bigrational: BigRational = match lhs {
						true => str_to_big_rational("1").unwrap(),
						false => str_to_big_rational("0").unwrap(),
					};
					Ok(Object::Float(lhs2bigrational + rhs))
				} else if let Object::Boolean(rhs) = other {
					let lhs2bigint: BigInt = match lhs {
						true => 1.to_bigint().unwrap(),
						false => 0.to_bigint().unwrap(),
					};
					let rhs2bigint: BigInt = match rhs {
						true => 1.to_bigint().unwrap(),
						false => 0.to_bigint().unwrap(),
					};
					Ok(Object::Integer(lhs2bigint + rhs2bigint))
				} else {
					Err(unsupported_operand_type("+", self.typer(), other.typer()))
				},
			Object::String(lhs) =>
				if let Object::String(rhs) = other {
					Ok(Object::String(format!("{}{}", lhs, rhs)))
				} else {
					Err(unsupported_operand_type("+", self.typer(), other.typer()))
				},
			_ => Err(unsupported_operand_type("+", self.typer(), other.typer())),
		}
	}
}

impl Sub for Object {
	type Output = Result<Self, Exception>;

	fn sub(self, other: Self) -> Result<Self, Exception> {
		match &self {
			Object::Integer(lhs) =>
				if let Object::Integer(rhs) = other {
					Ok(Object::Integer(lhs - rhs))
				} else if let Object::Float(rhs) = other {
					Ok(Object::Float(str_to_big_rational(&lhs.to_string()).unwrap() - rhs))
				} else if let Object::Boolean(rhs) = other {
					let rhs2bigint: BigInt = match rhs {
						true => 1.to_bigint().unwrap(),
						false => 0.to_bigint().unwrap(),
					};
					Ok(Object::Integer(lhs - rhs2bigint))
				} else {
					Err(unsupported_operand_type("-", self.typer(), other.typer()))
				},
			Object::Float(lhs) =>
				if let Object::Integer(rhs) = other {
					Ok(Object::Float(lhs - str_to_big_rational(&rhs.to_string()).unwrap()))
				} else if let Object::Float(rhs) = other {
					Ok(Object::Float(lhs - rhs))
				} else if let Object::Boolean(rhs) = other {
					let rhs2bigrational: BigRational = match rhs {
						true => str_to_big_rational("1").unwrap(),
						false => str_to_big_rational("0").unwrap(),
					};
					Ok(Object::Float(lhs - rhs2bigrational))
				} else {
					Err(unsupported_operand_type("-", self.typer(), other.typer()))
				},
			Object::Boolean(lhs) =>
				if let Object::Integer(rhs) = other {
					let lhs2bigint: BigInt = match lhs {
						true => 1.to_bigint().unwrap(),
						false => 0.to_bigint().unwrap(),
					};
					Ok(Object::Integer(lhs2bigint - rhs))
				} else if let Object::Float(rhs) = other {
					let lhs2bigrational: BigRational = match lhs {
						true => str_to_big_rational("1").unwrap(),
						false => str_to_big_rational("0").unwrap(),
					};
					Ok(Object::Float(lhs2bigrational - rhs))
				} else if let Object::Boolean(rhs) = other {
					let lhs2bigint: BigInt = match lhs {
						true => 1.to_bigint().unwrap(),
						false => 0.to_bigint().unwrap(),
					};
					let rhs2bigint: BigInt = match rhs {
						true => 1.to_bigint().unwrap(),
						false => 0.to_bigint().unwrap(),
					};
					Ok(Object::Integer(lhs2bigint - rhs2bigint))
				} else {
					Err(unsupported_operand_type("-", self.typer(), other.typer()))
				},
			_ => Err(unsupported_operand_type("-", self.typer(), other.typer())),
		}
	}
}

impl Mul for Object {
	type Output = Result<Self, Exception>;

	fn mul(self, other: Self) -> Result<Self, Exception> {
		match &self {
			Object::Integer(lhs) =>
				if let Object::Integer(rhs) = other {
					Ok(Object::Integer(lhs * rhs))
				} else if let Object::Float(rhs) = other {
					Ok(Object::Float(str_to_big_rational(&lhs.to_string()).unwrap() * rhs))
				} else if let Object::Boolean(rhs) = other {
					let rhs2bigint: BigInt = match rhs {
						true => 1.to_bigint().unwrap(),
						false => 0.to_bigint().unwrap(),
					};
					Ok(Object::Integer(lhs * rhs2bigint))
				} else {
					Err(unsupported_operand_type("*", self.typer(), other.typer()))
				},
			Object::Float(lhs) =>
				if let Object::Integer(rhs) = other {
					Ok(Object::Float(lhs * str_to_big_rational(&rhs.to_string()).unwrap()))
				} else if let Object::Float(rhs) = other {
					Ok(Object::Float(lhs * rhs))
				} else if let Object::Boolean(rhs) = other {
					let rhs2bigrational: BigRational = match rhs {
						true => str_to_big_rational("1").unwrap(),
						false => str_to_big_rational("0").unwrap(),
					};
					Ok(Object::Float(lhs * rhs2bigrational))
				} else {
					Err(unsupported_operand_type("*", self.typer(), other.typer()))
				},
			Object::Boolean(lhs) =>
				if let Object::Integer(rhs) = other {
					let lhs2bigint: BigInt = match lhs {
						true => 1.to_bigint().unwrap(),
						false => 0.to_bigint().unwrap(),
					};
					Ok(Object::Integer(lhs2bigint * rhs))
				} else if let Object::Float(rhs) = other {
					let lhs2bigrational: BigRational = match lhs {
						true => str_to_big_rational("1").unwrap(),
						false => str_to_big_rational("0").unwrap(),
					};
					Ok(Object::Float(lhs2bigrational * rhs))
				} else if let Object::Boolean(rhs) = other {
					let lhs2bigint: BigInt = match lhs {
						true => 1.to_bigint().unwrap(),
						false => 0.to_bigint().unwrap(),
					};
					let rhs2bigint: BigInt = match rhs {
						true => 1.to_bigint().unwrap(),
						false => 0.to_bigint().unwrap(),
					};
					Ok(Object::Integer(lhs2bigint * rhs2bigint))
				} else {
					Err(unsupported_operand_type("*", self.typer(), other.typer()))
				},
			Object::String(lhs) =>
				if let Object::Integer(rhs) = other {
					let mut result_string: String = String::new();
					let mut i: BigInt = rhs;
					let zero: BigInt = 0.to_bigint().unwrap();
					while i > zero {
						i = i - 1;
						result_string = format!("{}{}", result_string, lhs);
					}
					Ok(Object::String(result_string))
				} else {
					Err(unsupported_operand_type("*", self.typer(), other.typer()))
				},
			_ => Err(unsupported_operand_type("*", self.typer(), other.typer())),
		}
	}
}

impl Div for Object {
	type Output = Result<Self, Exception>;

	fn div(self, other: Self) -> Result<Self, Exception> {
		match &self {
			Object::Integer(lhs) =>
				if let Object::Integer(rhs) = other {
					Ok(Object::Integer(lhs / rhs))
				} else if let Object::Float(rhs) = other {
					Ok(Object::Float(str_to_big_rational(&lhs.to_string()).unwrap() / rhs))
				} else if let Object::Boolean(rhs) = other {
					let rhs2bigint: BigInt = match rhs {
						true => 1.to_bigint().unwrap(),
						false => 0.to_bigint().unwrap(),
					};
					Ok(Object::Integer(lhs / rhs2bigint))
				} else {
					Err(unsupported_operand_type("/", self.typer(), other.typer()))
				},
			Object::Float(lhs) =>
				if let Object::Integer(rhs) = other {
					Ok(Object::Float(lhs / str_to_big_rational(&rhs.to_string()).unwrap()))
				} else if let Object::Float(rhs) = other {
					Ok(Object::Float(lhs / rhs))
				} else if let Object::Boolean(rhs) = other {
					let rhs2bigrational: BigRational = match rhs {
						true => str_to_big_rational("1").unwrap(),
						false => str_to_big_rational("0").unwrap(),
					};
					Ok(Object::Float(lhs / rhs2bigrational))
				} else {
					Err(unsupported_operand_type("/", self.typer(), other.typer()))
				},
			Object::Boolean(lhs) =>
				if let Object::Integer(rhs) = other {
					let lhs2bigint: BigInt = match lhs {
						true => 1.to_bigint().unwrap(),
						false => 0.to_bigint().unwrap(),
					};
					Ok(Object::Integer(lhs2bigint / rhs))
				} else if let Object::Float(rhs) = other {
					let lhs2bigrational: BigRational = match lhs {
						true => str_to_big_rational("1").unwrap(),
						false => str_to_big_rational("0").unwrap(),
					};
					Ok(Object::Float(lhs2bigrational / rhs))
				} else if let Object::Boolean(rhs) = other {
					let lhs2bigint: BigInt = match lhs {
						true => 1.to_bigint().unwrap(),
						false => 0.to_bigint().unwrap(),
					};
					let rhs2bigint: BigInt = match rhs {
						true => 1.to_bigint().unwrap(),
						false => 0.to_bigint().unwrap(),
					};
					Ok(Object::Integer(lhs2bigint / rhs2bigint))
				} else {
					Err(unsupported_operand_type("/", self.typer(), other.typer()))
				},
			_ => Err(unsupported_operand_type("/", self.typer(), other.typer())),
		}
	}
}

impl PartialOrd for Object {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		match self {
			Object::Integer(lhs) =>
				if let Object::Integer(rhs) = other {
					Some(lhs.cmp(rhs))
				} else if let Object::Float(rhs) = other {
					Some(str_to_big_rational(&lhs.to_string()).unwrap().cmp(rhs))
				} else if let Object::Boolean(rhs) = other {
					let rhs2bigint: BigInt = match rhs {
						true => 1.to_bigint().unwrap(),
						false => 0.to_bigint().unwrap(),
					};
					Some(lhs.cmp(&rhs2bigint))
				} else {
					None
				},
			Object::Float(lhs) =>
				if let Object::Integer(rhs) = other {
					Some(lhs.cmp(&str_to_big_rational(&rhs.to_string()).unwrap()))
				} else if let Object::Float(rhs) = other {
					Some(lhs.cmp(rhs))
				} else if let Object::Boolean(rhs) = other {
					let rhs2bigrational: BigRational = match rhs {
						true => str_to_big_rational("1").unwrap(),
						false => str_to_big_rational("0").unwrap(),
					};
					Some(rhs2bigrational.cmp(&lhs))
				} else {
					None
				},
			Object::Boolean(lhs) =>
				if let Object::Integer(rhs) = other {
					let lhs2bigint: BigInt = match lhs {
						true => 1.to_bigint().unwrap(),
						false => 0.to_bigint().unwrap(),
					};
					Some(lhs2bigint.cmp(&rhs))
				} else if let Object::Float(rhs) = other {
					let lhs2bigrational: BigRational = match lhs {
						true => str_to_big_rational("1").unwrap(),
						false => str_to_big_rational("0").unwrap(),
					};
					Some(lhs2bigrational.cmp(&rhs))
				} else if let Object::Boolean(rhs) = other {
					Some(lhs.cmp(&rhs))
				} else {
					None
				},
			_ => None,
		}
	}
}

fn unsupported_operand_type(infixtype: &str, lhstype: &str, rhstype: &str) -> Exception {
	Exception::in_runtime(Except::type_(format!(
		"unsupported operand type(s) for {}: '{}' and '{}'",
		infixtype, lhstype, rhstype
	)))
}
