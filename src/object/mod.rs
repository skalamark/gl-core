// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;
use downcast_rs::{impl_downcast, DowncastSync};

mod boolean;
mod float;
mod function;
mod function_rust;
mod hashmap;
mod integer;
mod null;
mod string;
mod vector;

pub use boolean::Boolean;
pub use float::Float;
pub use function::Function;
pub use function_rust::FunctionRust;
pub use hashmap::HashMapGL;
pub use integer::Integer;
pub use null::Null;
pub use string::StringLiteral;
pub use vector::Vector;

pub trait Object: std::fmt::Display + std::fmt::Debug + DowncastSync {
	fn typer(&self) -> &str {
		"object"
	}

	fn getattribute(&mut self, _: String) -> Option<Box<dyn Object>> {
		None
	}

	fn call(
		&mut self, attr: String, _: Vec<Arc<Mutex<Box<dyn Object>>>>, module: String,
		position: Position,
	) -> Result<Arc<Mutex<Box<dyn Object>>>, Exception> {
		let mut exception: Exception = Exception::new(
			Except::name(format!(
				"'{}' object has no attribute '{}'",
				self.typer(),
				&attr
			)),
			true,
		);
		exception.push(ExceptionPoint::new(module.clone(), position));
		Err(exception)
	}
}

impl Hash for dyn Object {
	fn hash<H: Hasher>(&self, state: &mut H) {
		if let Some(integer_object) = self.downcast_ref::<Integer>() {
			integer_object.hash(state)
		} else if let Some(float_object) = self.as_any().downcast_ref::<Float>() {
			float_object.hash(state)
		} else if let Some(boolean_object) = self.as_any().downcast_ref::<Boolean>() {
			boolean_object.hash(state)
		} else if let Some(string_object) = self.as_any().downcast_ref::<StringLiteral>() {
			string_object.hash(state)
		} else {
			"".hash(state)
		}
	}
}

impl PartialEq for dyn Object {
	fn eq(&self, other: &Self) -> bool {
		self == other
	}
}

impl Eq for dyn Object {}

impl_downcast!(sync Object);
