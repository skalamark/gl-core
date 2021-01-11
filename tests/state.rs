// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

extern crate gl_core;

use gl_core::state::{EnvState, ProgramState};

#[test]
fn program_state_new() {
	let _program_state: ProgramState = ProgramState::new();
}

#[test]
fn env_state_new() {
	let _env_state: EnvState = EnvState::new();
}
