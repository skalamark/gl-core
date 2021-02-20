// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

pub struct FunctionRust {
	pub value: fn(
		Vec<Arc<Mutex<Box<dyn Object>>>>,
		String,
		Position,
	) -> Result<Arc<Mutex<Box<dyn Object>>>, Exception>,
}

impl Object for FunctionRust {}

impl std::fmt::Display for FunctionRust {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "")
	}
}

impl std::fmt::Debug for FunctionRust {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "")
	}
}

impl FunctionRust {
	pub fn new(
		value: fn(
			Vec<Arc<Mutex<Box<dyn Object>>>>,
			String,
			Position,
		) -> Result<Arc<Mutex<Box<dyn Object>>>, Exception>,
	) -> Self {
		FunctionRust { value }
	}

	pub fn call_value(
		&mut self, args: Vec<Arc<Mutex<Box<dyn Object>>>>, module: String, position: Position,
	) -> Result<Arc<Mutex<Box<dyn Object>>>, Exception> {
		(self.value)(args, module, position)
	}
}
