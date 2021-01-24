// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::error::{Exception, ExceptionError, ExceptionMain};
use crate::object::{BuiltinFn, Object};
use crate::position::Position;
use num::BigInt;
use rustyline::{error::ReadlineError, Editor};
use std::collections::HashMap;
use std::io::Write;

type ResultBuiltin = Result<Object, ExceptionMain>;

pub struct BuiltinsFns {}

impl BuiltinsFns {
	pub fn new() -> HashMap<String, Object> {
		let mut hashmap: HashMap<String, Object> = HashMap::new();
		let builtins = vec![
			add_builtin("print", -1, print_fn),
			add_builtin("println", -1, println_fn),
			add_builtin("input", 1, input_fn),
			add_builtin("len", 1, len_fn),
			add_builtin("push", 2, push_fn),
			add_builtin("insert", 3, insert_fn),
			add_builtin("Vec", 0, vec_new),
			add_builtin("HashMap", 0, hashmap_new),
		];

		for (name, object) in builtins {
			hashmap.insert(name, object);
		}
		hashmap
	}
}

fn add_builtin(name: &str, param_num: i32, func: BuiltinFn) -> (String, Object) {
	let name_string: String = String::from(name);
	(
		name_string.clone(),
		Object::Builtin(name_string, param_num, func),
	)
}

fn len_fn(args: Vec<Object>, module: String, position: Position) -> ResultBuiltin {
	Ok(match &args[0] {
		Object::String(string) => Object::Integer(BigInt::from(string.len())),
		Object::Vec(array) => Object::Integer(BigInt::from(array.len())),
		Object::HashMap(hashmap) => Object::Integer(BigInt::from(hashmap.len())),
		_ => {
			let mut exception: ExceptionMain = ExceptionMain::new(
				ExceptionError::invalid_syntax(format!("object has no len()")),
				false,
			);
			exception.push(Exception::new(module.clone(), position));
			return Err(exception);
		}
	})
}

fn print_fn(args: Vec<Object>, _: String, _: Position) -> ResultBuiltin {
	let mut fmt_string: String = String::new();
	for (i, object) in args.iter().enumerate() {
		match object {
			Object::String(string) => fmt_string.push_str(&format!("{}", string)),
			_ => fmt_string.push_str(&format!("{}", object)),
		}
		if i < args.len() - 1 {
			fmt_string.push_str(", ");
		}
	}
	print!("{}", fmt_string);
	std::io::stdout().flush().expect("");
	Ok(Object::Null)
}

fn println_fn(args: Vec<Object>, _: String, _: Position) -> ResultBuiltin {
	let mut fmt_string: String = String::new();
	for (i, object) in args.iter().enumerate() {
		match object {
			Object::String(string) => fmt_string.push_str(&format!("{}", string)),
			_ => fmt_string.push_str(&format!("{}", object)),
		}
		if i < args.len() - 1 {
			fmt_string.push_str(", ");
		}
	}
	println!("{}", fmt_string);
	Ok(Object::Null)
}

fn input_fn(args: Vec<Object>, module: String, position: Position) -> ResultBuiltin {
	let pretext: String = match &args[0] {
		Object::String(string) => format!("{}", string),
		_ => format!("{}", &args[0]),
	};

	let mut editor: Editor<()> = Editor::<()>::new();
	match editor.readline(&pretext) {
		Ok(buffer) => return Ok(Object::String(buffer)),
		Err(ReadlineError::Interrupted) => {
			let mut exception: ExceptionMain =
				ExceptionMain::new(ExceptionError::keyboard_interrupt(format!("")), true);
			exception.push(Exception::new(module.clone(), position));
			return Err(exception);
		}
		Err(ReadlineError::Eof) => {
			let mut exception: ExceptionMain =
				ExceptionMain::new(ExceptionError::eof(format!("")), true);
			exception.push(Exception::new(module.clone(), position));
			return Err(exception);
		}
		Err(err) => {
			let mut exception: ExceptionMain =
				ExceptionMain::new(ExceptionError::eof(format!("{}", err)), true);
			exception.push(Exception::new(module.clone(), position));
			return Err(exception);
		}
	}
}

fn vec_new(_: Vec<Object>, _: String, _: Position) -> ResultBuiltin {
	Ok(Object::Vec(Vec::new()))
}

fn hashmap_new(_: Vec<Object>, _: String, _: Position) -> ResultBuiltin {
	Ok(Object::HashMap(HashMap::new()))
}

fn push_fn(args: Vec<Object>, module: String, position: Position) -> ResultBuiltin {
	match &args[0] {
		Object::Vec(array) => {
			let mut arrayclone = array.clone();
			arrayclone.push(args[1].clone());
			Ok(Object::Vec(arrayclone))
		}
		_ => {
			let mut exception: ExceptionMain = ExceptionMain::new(
				ExceptionError::invalid_syntax(format!("object has no push()")),
				false,
			);
			exception.push(Exception::new(module.clone(), position));
			return Err(exception);
		}
	}
}

fn insert_fn(args: Vec<Object>, module: String, position: Position) -> ResultBuiltin {
	match &args[0] {
		Object::HashMap(hashmap) => {
			let mut hashmapclone = hashmap.clone();
			hashmapclone.insert(args[1].clone(), args[2].clone());
			Ok(Object::HashMap(hashmapclone))
		}
		_ => {
			let mut exception: ExceptionMain = ExceptionMain::new(
				ExceptionError::invalid_syntax(format!("object has no insert()")),
				false,
			);
			exception.push(Exception::new(module.clone(), position));
			return Err(exception);
		}
	}
}
