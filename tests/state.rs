// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

extern crate gl_core;

use gl_core::preludes::*;

#[test]
fn program_state_new() {
	let _program_state: ProgramState = ProgramState::new("tests/state/program_state_new");
}
