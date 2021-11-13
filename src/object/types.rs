// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use core::fmt::{Display, Formatter};
use core::hash::{Hash, Hasher};

use crate::preludes::*;

pub type FnNative = fn(&mut Interpreter, Vec<Object>) -> GLResult;
pub type FNStructRust = fn(Box<dyn ObjectTrait>, Vec<Object>) -> GLResult;

pub type GInt = BigInt;
pub type GFloat = BigRational;
pub type GBoolean = bool;
pub type GString = String;
pub type GVec = Vec<Object>;
pub type GTuple = Vec<Object>;
pub type GHashMap = HashMap<Object, Object>;

#[derive(PartialEq)]
pub struct GFunction {
	pub name: Option<String>,
	pub params: Vec<String>,
	pub body: Block,
}

pub struct GFunctionNative {
	pub name: Option<String>,
	pub params_len: i32,
	pub body: FnNative,
}

#[derive(PartialEq)]
pub struct GModule {
	pub name: String,
	pub path: String,
	pub context: Scope,
}

#[derive(PartialEq)]
pub struct GModuleNative {
	pub name: String,
	pub context: HashMap<String, Object>,
}

#[derive(PartialEq)]
pub enum Object {
	Null,
	Integer(GInt),
	Float(GFloat),
	Boolean(GBoolean),
	String(GString),
	Vec(GVec),
	Tuple(GTuple),
	HashMap(GHashMap),
	Fn(GFunction),
	FnNative(GFunctionNative),
	Module(GModule),
	ModuleNative(GModuleNative),
	ModuleDynLibrary(ModuleDynLibrary),
	StructRust(StructRust),
}

// Object

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
			Object::Fn(..) => "Fn",
			Object::FnNative(..) => "Fn",
			Object::Module(..) => "Module",
			Object::ModuleNative(..) => "Module",
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
			Object::Fn(gfn) => write!(f, "{}", gfn),
			Object::FnNative(gfn) => write!(f, "{}", gfn),
			Object::Module(gm) => write!(f, "{}", gm),
			Object::ModuleNative(gm) => write!(f, "{}", gm),
			Object::ModuleDynLibrary(module) => {
				write!(f, "<dynmodule '{}' from '{}'>", module.get_name(), module.get_path())
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
			Object::ModuleDynLibrary(ref m) => m.get_path().hash(state),
			_ => "".hash(state),
		}
	}
}

impl Clone for Object {
	fn clone(&self) -> Self {
		match self {
			Object::Null => Object::Null,
			Object::Integer(i) => Object::from(i.clone()),
			Object::Float(f) => Object::from(f.clone()),
			Object::Boolean(b) => Object::from(b.clone()),
			Object::String(s) => Object::from(s.clone()),
			Object::Vec(v) => Object::from(v.clone()),
			Object::Tuple(t) => Object::from(t.clone()),
			Object::HashMap(h) => Object::from(h.clone()),
			Object::Fn(gfn) => Object::from(gfn.clone()),
			Object::FnNative(gfn) => Object::from(gfn.clone()),
			Object::Module(gm) => Object::from(gm.clone()),
			Object::ModuleNative(gm) => Object::from(gm.clone()),
			Object::ModuleDynLibrary(m) => Object::ModuleDynLibrary(m.clone()),
			Object::StructRust(s) => Object::StructRust(s.clone()),
		}
	}
}

impl From<GInt> for Object {
	fn from(data: GInt) -> Self { Self::Integer(data) }
}

impl From<GFloat> for Object {
	fn from(data: GFloat) -> Self { Self::Float(data) }
}

impl From<GBoolean> for Object {
	fn from(data: GBoolean) -> Self { Self::Boolean(data) }
}

impl From<GString> for Object {
	fn from(data: GString) -> Self { Self::String(data) }
}

impl From<GVec> for Object {
	fn from(data: GVec) -> Self { Self::Vec(data) }
}

impl From<GHashMap> for Object {
	fn from(data: GHashMap) -> Self { Self::HashMap(data) }
}

impl From<GFunction> for Object {
	fn from(data: GFunction) -> Self { Self::Fn(data) }
}

impl From<GFunctionNative> for Object {
	fn from(data: GFunctionNative) -> Self { Self::FnNative(data) }
}

impl From<GModule> for Object {
	fn from(data: GModule) -> Self { Self::Module(data) }
}

impl From<GModuleNative> for Object {
	fn from(data: GModuleNative) -> Self { Self::ModuleNative(data) }
}

// GFunction

impl GFunction {
	pub fn new(name: Option<String>, params: Vec<String>, body: Block) -> Self {
		Self { name, params, body }
	}
}

impl Display for GFunction {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		let mut params_string: String = String::new();
		for (i, param) in self.params.iter().enumerate() {
			params_string.push_str(param);
			if i < self.params.len() - 1 {
				params_string.push_str(", ");
			}
		}

		write!(
			f,
			"<function {} ({})>",
			match &self.name {
				Some(name_fn) => name_fn.clone(),
				None => format!("<anonymous>"),
			},
			params_string
		)
	}
}

impl Clone for GFunction {
	fn clone(&self) -> Self { Self::new(self.name.clone(), self.params.clone(), self.body.clone()) }
}

// GFunctionNative

impl GFunctionNative {
	pub fn new(name: Option<String>, params_len: i32, body: FnNative) -> Self {
		Self { name, params_len, body }
	}
}

impl Display for GFunctionNative {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "<function {}>", match &self.name {
			Some(name_fn) => name_fn.clone(),
			None => format!("<anonymous>"),
		})
	}
}

impl Clone for GFunctionNative {
	fn clone(&self) -> Self { Self::new(self.name.clone(), self.params_len, self.body.clone()) }
}

impl PartialEq for GFunctionNative {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name && self.params_len == other.params_len
	}
}

// GModule

impl GModule {
	pub fn new(name: String, path: String, context: Scope) -> Self { Self { name, path, context } }

	pub fn get(&self, name: String) -> Option<Object> {
		match self.context.get(&name) {
			Some(o) => Some(o.clone()),
			None => None,
		}
	}
}

impl Display for GModule {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "<module '{}'>", self.name)
	}
}

impl Clone for GModule {
	fn clone(&self) -> Self {
		Self::new(self.name.clone(), self.path.clone(), self.context.clone())
	}
}

// GModuleNative

impl GModuleNative {
	pub fn new(name: String, context: HashMap<String, Object>) -> Self { Self { name, context } }

	pub fn get(&self, name: String) -> Option<Object> {
		match self.context.get(&name) {
			Some(o) => Some(o.clone()),
			None => None,
		}
	}
}

impl Display for GModuleNative {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "<module '{}'>", self.name)
	}
}

impl Clone for GModuleNative {
	fn clone(&self) -> Self { Self::new(self.name.clone(), self.context.clone()) }
}
