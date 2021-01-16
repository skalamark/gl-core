// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::builtins::BuiltinsFns;
use crate::object::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Env {
	store: HashMap<String, Object>,
	parent: Option<Rc<RefCell<Env>>>,
}

impl Env {
	pub fn new() -> Self {
		let mut store: HashMap<String, Object> = HashMap::new();
		Self::fill_env_with_builtins(&mut store);
		Self {
			store,
			parent: None,
		}
	}

	fn fill_env_with_builtins(hashmap: &mut HashMap<String, Object>) {
		let builtins_functions: BuiltinsFns = BuiltinsFns::new();
		let builtins: Vec<(String, Object)> = builtins_functions.get_builtins();
		for (name, object) in builtins {
			hashmap.insert(name, object);
		}
	}

	pub fn set(&mut self, identifier: &String, value: Object) {
		self.store.insert(identifier.clone(), value);
	}

	pub fn get(&self, identifier: &String) -> Option<Object> {
		match self.store.get(identifier) {
			Some(object) => Some(object.clone()),
			None => match self.parent {
				Some(ref parent_env) => parent_env.borrow().get(identifier),
				None => None,
			},
		}
	}
}
