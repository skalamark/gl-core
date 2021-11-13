// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use libloading::Library;

use crate::preludes::*;

pub struct ModuleDynLibrary {
	name: String,
	path: String,
	dynlibrary: Rc<RefCell<Library>>,
	env: Rc<RefCell<Scope>>,
}

impl ModuleDynLibrary {
	pub fn new<T: Into<String>>(
		name: T, path: T, dynlibrary: Rc<RefCell<Library>>, env: Rc<RefCell<Scope>>,
	) -> Self {
		Self { name: name.into(), path: path.into(), dynlibrary, env }
	}

	pub fn get_name(&self) -> String { self.name.clone() }

	pub fn get_path(&self) -> String { self.path.clone() }

	pub fn get_attr<T: Into<String>>(&self, name: T) -> Result<Object, libloading::Error> {
		let name: String = name.into();
		let attr = self.env.borrow().get(&name);

		match attr {
			Some(attr) => return Ok(attr),
			None => unsafe {
				self.env.borrow_mut().set(
					&name,
					Object::from(GFunctionNative::new(
						Some(name.clone()),
						-1,
						*self.dynlibrary.borrow().get::<FnNative>(name.as_bytes())?,
					)),
				);
				return self.get_attr(&name);
			},
		}
	}
}

impl Clone for ModuleDynLibrary {
	fn clone(&self) -> Self {
		Self {
			name: self.name.clone(),
			path: self.path.clone(),
			dynlibrary: self.dynlibrary.clone(),
			env: self.env.clone(),
		}
	}
}

impl PartialEq for ModuleDynLibrary {
	fn eq(&self, other: &Self) -> bool { self.path == other.path }
}

impl Eq for ModuleDynLibrary {}
