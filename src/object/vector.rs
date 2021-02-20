// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

pub struct Vector {
	pub value: Vec<Arc<Mutex<Box<dyn Object>>>>,
}

impl Object for Vector {}

impl std::fmt::Display for Vector {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut fmt_string: String = String::new();
		fmt_string.push('[');
		for (i, objs) in self.value.iter().enumerate() {
			fmt_string.push_str(&format!("{}", &objs.lock().unwrap()));
			if i < self.value.len() - 1 {
				fmt_string.push_str(", ");
			}
		}
		fmt_string.push(']');
		write!(f, "{}", fmt_string)
	}
}

impl std::fmt::Debug for Vector {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self)
	}
}

impl Vector {
	pub fn new() -> Self {
		Self { value: Vec::new() }
	}

	pub fn new_with_value(value: Vec<Arc<Mutex<Box<dyn Object>>>>) -> Self {
		Self { value }
	}
}
