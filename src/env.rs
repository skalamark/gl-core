// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

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
		Self {
			store: HashMap::new(),
			parent: None,
		}
	}

	pub fn from(store: HashMap<String, Object>) -> Self {
		Self {
			store,
			parent: None,
		}
	}

	pub fn new_with_parent(parent: Rc<RefCell<Env>>) -> Self {
		Self {
			store: HashMap::new(),
			parent: Some(parent),
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
