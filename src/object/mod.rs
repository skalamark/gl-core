// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use core::cmp::PartialOrd;
use core::ops::{Add, Div, Mul, Sub};
use std::hash::{Hash, Hasher};

pub use dynlibrary::ModuleDynLibrary;
pub use structrust::{ObjectTrait, StructRust};

use crate::preludes::*;

mod dynlibrary;
mod structrust;

pub type FnRust = fn(Vec<Object>) -> Result<Object, Exception>;
pub type FNStructRust = fn(Box<dyn ObjectTrait>, Vec<Object>) -> Result<Object, Exception>;

#[derive(PartialEq)]
pub enum Object {
	Null,
	Integer(BigInt),
	Float(BigRational),
	Boolean(bool),
	String(String),
	Vec(Vec<Object>),
	Tuple(Vec<Object>),
	HashMap(HashMap<Object, Object>),
	Builtin(String, i32, FnRust),
	Fn(Option<String>, Vec<String>, Block),
	FnRust(Option<String>, i32, FnRust),
	ModuleRust(String, HashMap<String, Object>),
	ModuleDynLibrary(ModuleDynLibrary),
	StructRust(StructRust),
}

impl Object {
	pub fn typer(&self) -> &str {
		match self {
			Object::Null => "Null",
			Object::Integer(_) => "Integer",
			Object::Float(_) => "Float",
			Object::Boolean(_) => "Boolean",
			Object::String(_) => "String",
			Object::Vec(_) => "Vec",
			Object::Tuple(_) => "Tuple",
			Object::HashMap(_) => "HashMap",
			Object::Builtin(..) => "Builtin",
			Object::Fn(..) => "Fn",
			Object::FnRust(..) => "Fn",
			Object::ModuleRust(..) => "Module",
			Object::ModuleDynLibrary(_) => "Module",
			Object::StructRust(_) => "ObjectRust",
		}
	}
}

impl std::fmt::Display for Object {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Object::Null => write!(f, "null"),
			Object::Integer(integer) => write!(f, "{}", integer),
			Object::Float(float) => write!(f, "{}", big_rational_to_string(float.clone())),
			Object::Boolean(boolean) => write!(f, "{}", boolean),
			Object::String(string) => write!(f, "{:?}", string),
			Object::Vec(vector) => {
				let mut fmt_string: String = String::new();
				for (i, object) in vector.iter().enumerate() {
					fmt_string.push_str(&format!("{}", object));
					if i < vector.len() - 1 {
						fmt_string.push_str(", ");
					}
				}
				write!(f, "[{}]", fmt_string)
			},
			Object::Tuple(values) => {
				let mut fmt_string: String = String::new();
				for (i, object) in values.iter().enumerate() {
					fmt_string.push_str(&format!("{}", object));
					if i < values.len() - 1 {
						fmt_string.push_str(", ");
					}
				}
				write!(f, "({})", fmt_string)
			},
			Object::HashMap(hashmap) => {
				let mut fmt_string: String = String::new();
				for (i, (key, value)) in hashmap.iter().enumerate() {
					fmt_string.push_str(&format!("{}: {}", key, value));
					if i < hashmap.len() - 1 {
						fmt_string.push_str(", ");
					}
				}
				write!(f, "{{{}}}", fmt_string)
			},
			Object::Builtin(name, ..) => write!(f, "<built-in function {}>", name),
			Object::Fn(name, params, _) => {
				let name_fn: String = match name {
					Some(name_fn) => name_fn.clone(),
					None => format!("<anonymous>"),
				};
				let mut params_string: String = String::new();
				for (i, param) in params.iter().enumerate() {
					params_string.push_str(param);
					if i < params.len() - 1 {
						params_string.push_str(", ");
					}
				}
				write!(f, "<function {} ({})>", name_fn, params_string)
			},
			Object::FnRust(name, ..) => {
				let name_fn: String = match name {
					Some(name_fn) => name_fn.clone(),
					None => format!("<anonymous>"),
				};
				write!(f, "<function {}>", name_fn)
			},
			Object::ModuleDynLibrary(module) => {
				write!(f, "<dynmodule '{}' from '{}'>", module.get_name(), module.get_path())
			},
			Object::ModuleRust(name, _) => {
				write!(f, "<module '{}'>", name)
			},
			Object::StructRust(s) => {
				write!(f, "{}", s)
			},
		}
	}
}

impl Eq for Object {}

impl Hash for Object {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match &self {
			Object::Integer(ref i) => i.hash(state),
			Object::Float(ref f) => f.hash(state),
			Object::Boolean(ref b) => b.hash(state),
			Object::String(ref s) => s.hash(state),
			Object::Vec(ref v) => v.hash(state),
			Object::Tuple(ref t) => t.hash(state),
			Object::Builtin(ref b, ..) => b.hash(state),
			Object::ModuleRust(ref m, ..) => m.hash(state),
			Object::ModuleDynLibrary(ref m) => m.get_path().hash(state),
			_ => "".hash(state),
		}
	}
}

impl Clone for Object {
	fn clone(&self) -> Self {
		match self {
			Object::Null => Object::Null,
			Object::Integer(i) => Object::Integer(i.clone()),
			Object::Float(f) => Object::Float(f.clone()),
			Object::Boolean(b) => Object::Boolean(b.clone()),
			Object::String(s) => Object::String(s.clone()),
			Object::Vec(v) => Object::Vec(v.clone()),
			Object::Tuple(t) => Object::Tuple(t.clone()),
			Object::HashMap(h) => Object::HashMap(h.clone()),
			Object::Builtin(name, len_args, func) =>
				Object::Builtin(name.clone(), len_args.clone(), func.clone()),
			Object::Fn(name, params, body) =>
				Object::Fn(name.clone(), params.clone(), body.clone()),
			Object::FnRust(name, len_args, func) =>
				Object::FnRust(name.clone(), len_args.clone(), func.clone()),
			Object::ModuleRust(name, env) => Object::ModuleRust(name.clone(), env.clone()),
			Object::ModuleDynLibrary(m) => Object::ModuleDynLibrary(m.clone()),
			Object::StructRust(s) => Object::StructRust(s.clone()),
		}
	}
}

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
