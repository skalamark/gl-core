// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;
use std::hash::{Hash, Hasher};

pub type BuiltinFn = fn(Vec<Object>, String, Position) -> Result<Object, Exception>;

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
	Null,
	Integer(BigInt),
	Float(BigRational),
	Boolean(bool),
	String(String),
	Vec(Vec<Object>),
	HashMap(HashMap<Object, Object>),
	Builtin(String, i32, BuiltinFn),
	Fn(Option<String>, Vec<String>, Block),
}

impl std::fmt::Display for Object {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Object::Null => write!(f, "null"),
			Object::Integer(integer) => write!(f, "{}", integer),
			Object::Float(float) => write!(f, "{}", float),
			Object::Boolean(boolean) => write!(f, "{}", boolean),
			Object::String(string) => write!(f, "{:?}", string),
			Object::Vec(vector) => {
				let mut fmt_string: String = String::new();
				fmt_string.push('[');
				for (i, object) in vector.iter().enumerate() {
					fmt_string.push_str(&format!("{}", object));
					if i < vector.len() - 1 {
						fmt_string.push_str(", ");
					}
				}
				fmt_string.push(']');
				write!(f, "{}", fmt_string)
			}
			Object::HashMap(hashmap) => {
				let mut fmt_string: String = String::new();
				fmt_string.push('{');
				for (i, (key, value)) in hashmap.iter().enumerate() {
					fmt_string.push_str(&format!("{}: {}", key, value));
					if i < hashmap.len() - 1 {
						fmt_string.push_str(", ");
					}
				}
				fmt_string.push('}');
				write!(f, "{}", fmt_string)
			}
			Object::Builtin(_, _, _) => {
				write!(f, "<built-in function>")
			}
			Object::Fn(name, params, _) => {
				let mut fmt_string: String = String::new();
				fmt_string.push_str(&format!("<function "));
				let name_fn: String = match name {
					Some(name_fn) => name_fn.clone(),
					None => format!("<anonymous>"),
				};
				let mut params_string: String = String::new();
				params_string.push_str(&format!("("));
				for (i, param) in params.iter().enumerate() {
					params_string.push_str(&format!("{}", param));
					if i < params.len() - 1 {
						params_string.push_str(", ");
					}
				}
				params_string.push_str(&format!(")"));

				fmt_string.push_str(&format!("{} {}>", name_fn, params_string));
				write!(f, "{}", fmt_string)
			}
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
			_ => "".hash(state),
		}
	}
}
