// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::error::AnyError;
use num::BigInt;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub type BuiltinFn = fn(Vec<Object>) -> Result<Object, AnyError>;

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
	Null,
	Integer(BigInt),
	Boolean(bool),
	String(String),
	Vec(Vec<Object>),
	HashMap(HashMap<Object, Object>),
	Builtin(String, i32, BuiltinFn),
}

impl std::fmt::Display for Object {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Object::Null => write!(f, "null"),
			Object::Integer(integer) => write!(f, "{}", integer),
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
		}
	}
}

impl Eq for Object {}

impl Hash for Object {
	fn hash<H: Hasher>(&self, state: &mut H) {
		match *self {
			Object::Integer(ref i) => i.hash(state),
			Object::Boolean(ref b) => b.hash(state),
			Object::String(ref s) => s.hash(state),
			_ => "".hash(state),
		}
	}
}
