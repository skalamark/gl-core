// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

extern crate gl_core;

use gl_core::env::Env;
use gl_core::state::{EnvState, ProgramState};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
	let _program_state: ProgramState = ProgramState::new(Rc::new(RefCell::new(Env::new())));
	let _env_state: EnvState = EnvState::new(Rc::new(RefCell::new(Env::new())));
}
