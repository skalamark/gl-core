// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use libloading::Library;

use crate::preludes::*;

#[derive(Debug)]
pub struct ModuleDynLibrary {
	name: String,
	path: String,
	dynlibrary: Library,
	functions: HashMap<String, FnRust>,
}

impl ModuleDynLibrary {
	pub fn new<T: Into<String>>(
		name: T, path: T, dynlibrary: Library, functions: HashMap<String, FnRust>,
	) -> Self {
		Self { name: name.into(), path: path.into(), dynlibrary, functions }
	}

	pub fn get_name(&self) -> String { self.name.clone() }

	pub fn get_path(&self) -> String { self.path.clone() }

	pub fn get_function<T: Into<String>>(&mut self, name: T) -> Result<FnRust, libloading::Error> {
		let name: String = name.into();

		match self.functions.get(&name) {
			Some(function) => Ok(function.clone()),
			None => unsafe {
				match self.dynlibrary.get::<FnRust>(name.as_bytes()) {
					Ok(function) => {
						self.functions.insert(name.clone(), *function);
						return self.get_function(name.clone());
					},
					Err(err) => {
						return Err(err);
					},
				}
			},
		}
	}
}

impl Clone for ModuleDynLibrary {
	fn clone(&self) -> Self {
		Self {
			name: self.name.clone(),
			path: self.path.clone(),
			dynlibrary: unsafe { Library::new(self.path.clone()).unwrap() },
			functions: self.functions.clone(),
		}
	}
}

impl PartialEq for ModuleDynLibrary {
	fn eq(&self, other: &Self) -> bool { self.path == other.path }
}

impl Eq for ModuleDynLibrary {}
