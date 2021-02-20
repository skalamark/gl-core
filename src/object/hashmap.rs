// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

pub struct HashMapGL {
	pub value: HashMap<String, Arc<Mutex<Box<dyn Object>>>>,
}

impl Object for HashMapGL {}

impl std::fmt::Display for HashMapGL {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut fmt_string: String = String::new();
		fmt_string.push('{');
		for (i, (key, value)) in self.value.iter().enumerate() {
			fmt_string.push_str(&format!("{}: {}", key, value.lock().unwrap()));
			if i < self.value.len() - 1 {
				fmt_string.push_str(", ");
			}
		}
		fmt_string.push('}');
		write!(f, "{}", fmt_string)
	}
}

impl std::fmt::Debug for HashMapGL {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self)
	}
}

impl HashMapGL {
	pub fn new() -> Self {
		Self {
			value: HashMap::new(),
		}
	}

	pub fn new_with_value(value: HashMap<String, Arc<Mutex<Box<dyn Object>>>>) -> Self {
		Self { value }
	}
}
