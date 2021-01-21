// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::object::Object;
use crate::{builtins, env::Env};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct ProgramState {
	pub env: EnvState,
}

pub struct EnvState {
	pub crate_module: String,
	pub modules: HashMap<String, Rc<RefCell<Env>>>,
}

impl ProgramState {
	pub fn new() -> Self {
		Self {
			env: EnvState::new(),
		}
	}
}

impl EnvState {
	pub fn new() -> Self {
		Self {
			crate_module: String::new(),
			modules: HashMap::new(),
		}
	}

	pub fn add_module(&mut self, name: String) {
		self.modules.insert(
			name,
			Rc::new(RefCell::new(Env::from(builtins::BuiltinsFns::new()))),
		);
	}

	pub fn get(&self, identifier: &String, module: &String) -> Option<Object> {
		if module == "crate" {
			self.modules[&self.crate_module].borrow().get(identifier)
		} else {
			self.modules[module].borrow().get(identifier)
		}
	}

	pub fn set(&self, identifier: &String, value: Object, module: &String) {
		if module == "crate" {
			self.modules[&self.crate_module]
				.borrow_mut()
				.set(identifier, value)
		} else {
			self.modules[module].borrow_mut().set(identifier, value)
		}
	}
}
