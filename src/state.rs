// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

pub struct ProgramState {
	crate_module: String,
	pub modules: HashMap<String, Rc<RefCell<Env>>>,
	std: Rc<RefCell<Env>>,
}

impl ProgramState {
	pub fn new<T: Into<String>>(crate_module: T) -> Self {
		Self {
			crate_module: crate_module.into(),
			modules: HashMap::new(),
			std: Rc::new(RefCell::new(Env::new())),
		}
	}

	pub fn with_std<T: Into<String>>(crate_module: T, std: Env) -> Self {
		Self {
			crate_module: crate_module.into(),
			modules: HashMap::new(),
			std: Rc::new(RefCell::new(std)),
		}
	}

	pub fn add_module<T: Into<String>>(&mut self, name: T) {
		self.modules.insert(name.into(), Rc::new(RefCell::new(Env::from_parent(self.std.clone()))));
	}

	pub fn add_module_with_env<T: Into<String>>(&mut self, name: T, env: Env) {
		self.modules.insert(name.into(), Rc::new(RefCell::new(env)));
	}

	pub fn get<T: Into<String>>(&self, identifier: T, module: T) -> Option<Object> {
		let identifier: String = identifier.into();
		let module: String = module.into();

		if let Some(oo) = if module == "crate" {
			self.modules[&self.crate_module].borrow().get(&identifier)
		} else {
			self.modules[&module].borrow().get(&identifier)
		} {
			return Some(oo);
		} else {
			self.std.borrow().get(&identifier)
		}
	}

	pub fn set<T: Into<String>>(&self, identifier: T, value: Object, module: T) {
		let module: String = module.into();

		if module == "crate" {
			self.modules[&self.crate_module].borrow_mut().set(identifier, value)
		} else {
			self.modules[&module].borrow_mut().set(identifier, value)
		}
	}
}
