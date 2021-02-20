// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

pub mod ast; // l
pub mod env;
pub mod error; // l
pub mod lexer; // l
pub mod object;
pub mod parser; // l
pub mod position; // l
pub mod source;
pub mod state;
pub mod token; // l

pub mod preludes {
	pub use arc_map::ArcMap;
	pub use num::bigint::ToBigInt;
	pub use num::{BigInt, BigRational, FromPrimitive, ToPrimitive};
	pub use std::cell::RefCell;
	pub use std::collections::HashMap;
	pub use std::hash::{Hash, Hasher};
	pub use std::rc::Rc;
	pub use std::sync::{Arc, Mutex};

	pub type ResultLexer = Result<(), Exception>;

	pub use crate::ast::*;
	pub use crate::env::Env;
	pub use crate::error::*;
	pub use crate::lexer::Lexer;
	pub use crate::object::*;
	pub use crate::parser::Parser;
	pub use crate::position::Position;
	pub use crate::source::Source;
	pub use crate::state::*;
	pub use crate::token::*;
}
