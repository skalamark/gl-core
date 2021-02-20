// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

pub struct Float {
	pub value: BigRational,
}

impl Object for Float {}

impl std::fmt::Display for Float {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", &self.value)
	}
}

impl std::fmt::Debug for Float {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", &self.value)
	}
}

impl Hash for Float {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.value.hash(state)
	}
}

impl Float {
	pub fn new(value: BigRational) -> Self {
		Self { value }
	}
}
