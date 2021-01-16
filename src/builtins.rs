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

fn add_builtin(name: &str, param_num: i32, func: BuiltinFn) -> (String, Object) {
	let name_string: String = String::from(name);
	(
		name_string.clone(),
		Object::Builtin(name_string, param_num, func),
	)
}

fn print_fn(args: Vec<Object>) -> Result<Object, AnyError> {
	let mut fmt_string: String = String::new();
	for (i, object) in args.iter().enumerate() {
		fmt_string.push_str(&format!("{}", object));
		if i < args.len() - 1 {
			fmt_string.push_str(", ");
		}
	}
	print!("{}", fmt_string);
	Ok(Object::Null)
}

fn println_fn(args: Vec<Object>) -> Result<Object, AnyError> {
	let mut fmt_string: String = String::new();
	for (i, object) in args.iter().enumerate() {
		fmt_string.push_str(&format!("{}", object));
		if i < args.len() - 1 {
			fmt_string.push_str(", ");
		}
	}
	println!("{}", fmt_string);
	Ok(Object::Null)
}
