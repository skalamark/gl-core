// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

pub struct Null {}

impl Object for Null {}

impl std::fmt::Display for Null {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "null")
	}
}

impl std::fmt::Debug for Null {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "null")
	}
}

impl Null {
	pub fn new() -> Self {
		Self {}
	}
}
