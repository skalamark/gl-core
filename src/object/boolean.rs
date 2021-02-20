// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

pub struct Boolean {
	pub value: bool,
}

impl Object for Boolean {}

impl std::fmt::Display for Boolean {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.value)
	}
}

impl std::fmt::Debug for Boolean {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.value)
	}
}

impl Hash for Boolean {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.value.hash(state)
	}
}

impl Boolean {
	pub fn new(value: bool) -> Self {
		Self { value }
	}
}
