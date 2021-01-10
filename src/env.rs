// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::object::Object;
use std::collections::HashMap;

pub struct Env {
	data: HashMap<String, Object>,
}

impl Env {
	pub fn new() -> Self {
		Self {
			data: HashMap::new(),
		}
	}
}
