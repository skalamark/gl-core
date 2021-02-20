// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

pub struct Integer {
	pub value: BigInt,
}

impl Object for Integer {}

impl std::fmt::Display for Integer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", &self.value)
	}
}

impl std::fmt::Debug for Integer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", &self.value)
	}
}

impl Hash for Integer {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.value.hash(state)
	}
}

impl Integer {
	pub fn new(value: BigInt) -> Self {
		Self { value }
	}
}
