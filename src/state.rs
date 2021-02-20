// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::env::Env;
use crate::object::Object;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ProgramState {
	pub env: EnvState,
}

pub struct EnvState {
	pub crate_module: String,
	pub builtins_module: Arc<Mutex<Env>>,
	pub modules: HashMap<String, Arc<Mutex<Env>>>,
}

impl ProgramState {
	pub fn new(builtins_module: Arc<Mutex<Env>>) -> Self {
		Self {
			env: EnvState::new(builtins_module),
		}
	}
}

impl EnvState {
	pub fn new(builtins_module: Arc<Mutex<Env>>) -> Self {
		Self {
			crate_module: String::new(),
			builtins_module,
			modules: HashMap::new(),
		}
	}

	pub fn add_module(&mut self, name: String) {
		self.modules.insert(
			name,
			Arc::new(Mutex::new(Env::new_with_parent(Arc::clone(
				&self.builtins_module,
			)))),
		);
	}

	pub fn get(&self, identifier: &String, module: &String) -> Option<Arc<Mutex<Box<dyn Object>>>> {
		if module == "crate" {
			let env = &mut *self.modules[&self.crate_module].lock().unwrap();
			env.get_clone_object(identifier)
		} else {
			let env = &mut *self.modules[module].lock().unwrap();
			env.get_clone_object(identifier)
		}
	}

	pub fn set(&self, identifier: &String, value: Arc<Mutex<Box<dyn Object>>>, module: &String) {
		if module == "crate" {
			let env = &mut *self.modules[module].lock().unwrap();
			env.set(identifier, value)
		} else {
			let env = &mut *self.modules[module].lock().unwrap();
			env.set(identifier, value)
		}
	}
}
