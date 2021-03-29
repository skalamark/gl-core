// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

pub struct Env {
	store: HashMap<String, Object>,
	parent: Option<Rc<RefCell<Env>>>,
}

impl Env {
	pub fn new() -> Self { Self { store: HashMap::new(), parent: None } }

	pub fn from_store(store: HashMap<String, Object>) -> Self { Self { store, parent: None } }

	pub fn from_parent(parent: Rc<RefCell<Env>>) -> Self {
		Self { store: HashMap::new(), parent: Some(parent) }
	}

	pub fn set<T: Into<String>>(&mut self, identifier: T, value: Object) {
		self.store.insert(identifier.into(), value);
	}

	pub fn get<T: Into<String>>(&self, identifier: T) -> Option<Object> {
		let identifier: String = identifier.into();

		match self.store.get(&identifier) {
			Some(object) => Some(object.clone()),
			None => match &self.parent {
				Some(parent_env) => parent_env.borrow().get(identifier),
				None => None,
			},
		}
	}
}
