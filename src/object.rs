// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;
use libloading::Library;
use std::hash::{Hash, Hasher};

pub type BuiltinFn = fn(Vec<Object>, String, Position) -> Result<Object, Exception>;

#[derive(Debug)]
pub struct ModuleDynLibrary {
	name: String,
	dynlibrary: Library,
	functions: HashMap<String, fn(HashMap<String, Object>) -> Result<(), Exception>>,
}

impl ModuleDynLibrary {
	pub fn new(
		name: String, dynlibrary: Library,
		functions: HashMap<String, fn(HashMap<String, Object>) -> Result<(), Exception>>,
	) -> Self {
		Self {
			name: name,
			dynlibrary,
			functions,
		}
	}

	pub fn get_name(&self) -> String {
		let mut name: String = self.name.clone();
		name = name.replace(".dll", "");
		name = name.replace(".so", "");
		name = name.replace("./", "");
		name
	}

	pub fn get_full_name(&self) -> String {
		self.name.clone()
	}

	pub fn get_function(
		&mut self, name: String,
	) -> Result<fn(HashMap<String, Object>) -> Result<(), Exception>, Exception> {
		match self.functions.get(&name) {
			Some(function) => Ok(function.clone()),
			None => unsafe {
				match self
					.dynlibrary
					.get::<fn(HashMap<String, Object>) -> Result<(), Exception>>(name.as_bytes())
				{
					Ok(function) => {
						self.functions.insert(name.clone(), *function);
						return self.get_function(name.clone());
					}
					Err(err) => {
						let exception: Exception =
							Exception::new(Except::unexpected_eof(format!("{}", err)), false);
						return Err(exception);
					}
				}
			},
		}
	}
}

impl Clone for ModuleDynLibrary {
	fn clone(&self) -> Self {
		Self {
			name: self.name.clone(),
			dynlibrary: unsafe { Library::new(self.name.clone()).unwrap() },
			functions: self.functions.clone(),
		}
	}
}

impl PartialEq for ModuleDynLibrary {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
	}
}

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
	ModuleDynLibrary(ModuleDynLibrary),
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
			Object::ModuleDynLibrary(module) => {
				write!(
					f,
					"<module <{}> from {}>",
					module.get_name(),
					module.get_full_name()
				)
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
