// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

pub struct StringLiteral {
	pub value: String,
}

impl Object for StringLiteral {}

impl std::fmt::Display for StringLiteral {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", &self.value)
	}
}

impl std::fmt::Debug for StringLiteral {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", &self.value)
	}
}

impl Hash for StringLiteral {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.value.hash(state)
	}
}

impl StringLiteral {
	pub fn new(value: String) -> Self {
		Self { value }
	}
}
