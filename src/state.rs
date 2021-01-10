// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::env::Env;
use std::collections::HashMap;

pub struct ProgramState {
	pub env: EnvState,
}

pub struct EnvState {
	pub modules: HashMap<String, Env>,
}

impl ProgramState {
	pub fn new() -> Self {
		Self {
			env: EnvState::new(),
		}
	}
}

impl EnvState {
	pub fn new() -> Self {
		Self {
			modules: HashMap::new(),
		}
	}
}
