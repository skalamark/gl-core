// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::error::AnyError;
use crate::object::{BuiltinFn, Object};

pub struct BuiltinsFns {}

impl BuiltinsFns {
	pub fn new() -> Self {
		BuiltinsFns {}
	}

	pub fn get_builtins(&self) -> Vec<(String, Object)> {
		vec![
			add_builtin("print", -1, print_fn),
			add_builtin("println", -1, println_fn),
		]
	}
}
