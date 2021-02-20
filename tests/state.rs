// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

extern crate gl_core;

use gl_core::env::Env;
use gl_core::state::{EnvState, ProgramState};
use std::sync::{Arc, Mutex};

#[test]
fn program_state_new() {
	let _program_state: ProgramState = ProgramState::new(Arc::new(Mutex::new(Env::new())));
}

#[test]
fn env_state_new() {
	let _env_state: EnvState = EnvState::new(Arc::new(Mutex::new(Env::new())));
}
