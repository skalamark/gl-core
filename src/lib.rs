// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

pub mod ast;
pub mod env;
pub mod error;
pub mod lexer;
pub mod object;
pub mod parser;
pub mod position;
pub mod source;
pub mod state;
pub mod token;
pub mod utils;

pub mod preludes {
	pub use std::cell::RefCell;
	pub use std::collections::HashMap;
	pub use std::rc::Rc;

	pub use num::bigint::ToBigInt;
	pub use num::{BigInt, BigRational, FromPrimitive, ToPrimitive};

	pub use crate::ast::*;
	pub use crate::env::*;
	pub use crate::error::*;
	pub use crate::lexer::*;
	pub use crate::object::*;
	pub use crate::parser::*;
	pub use crate::position::*;
	pub use crate::source::*;
	pub use crate::state::*;
	pub use crate::token::*;
	pub use crate::utils::*;
}
