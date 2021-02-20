// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Env {
	store: HashMap<String, Arc<Mutex<Box<dyn Object>>>>,
	parent: Option<Arc<Mutex<Env>>>,
}

impl Env {
	pub fn new() -> Self {
		Self {
			store: HashMap::new(),
			parent: None,
		}
	}

	pub fn from(store: HashMap<String, Arc<Mutex<Box<dyn Object>>>>) -> Self {
		Self {
			store,
			parent: None,
		}
	}

	pub fn new_with_parent(parent: Arc<Mutex<Env>>) -> Self {
		Self {
			store: HashMap::new(),
			parent: Some(parent),
		}
	}

	pub fn set(&mut self, identifier: &String, value: Arc<Mutex<Box<dyn Object>>>) {
		self.store.insert(identifier.clone(), value);
	}

	// pub fn get(&mut self, identifier: &String) -> Option<Box<dyn Object>> {
	// 	match self.store.remove(identifier) {
	// 		Some(a) => Some(Box::new(a.lock().unwrap())),
	// 		None => match &self.parent {
	// 			Some(parent_env) => {
	// 				let env = &mut *parent_env.lock().unwrap();
	// 				env.get(identifier)
	// 			}
	// 			None => None,
	// 		},
	// 	}
	// }

	pub fn get_clone_object(&mut self, identifier: &String) -> Option<Arc<Mutex<Box<dyn Object>>>> {
		match self.store.get(identifier) {
			Some(a) => Some(Arc::clone(a)),
			None => match &self.parent {
				Some(parent_env) => {
					let env = &mut *parent_env.lock().unwrap();
					env.get_clone_object(identifier)
				}
				None => None,
			},
		}
	}
}
