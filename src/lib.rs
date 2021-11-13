// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

pub mod ast;
pub mod error;
pub mod interpreter;
pub mod lexer;
pub mod macros;
pub mod object;
pub mod parser;
pub mod position;
pub mod source;
pub mod token;
pub mod utils;

#[macro_use]
pub mod preludes {
	pub type GLResult = Result<Object, Exception>;

	pub use std::cell::RefCell;
	pub use std::collections::HashMap;
	pub use std::rc::Rc;

	pub use num::bigint::ToBigInt;
	pub use num::{BigInt, BigRational, FromPrimitive, ToPrimitive};

	pub use crate::ast::*;
	pub use crate::error::*;
	pub use crate::interpreter::*;
	pub use crate::lexer::*;
	pub use crate::object::*;
	pub use crate::parser::*;
	pub use crate::position::*;
	pub use crate::source::*;
	pub use crate::token::*;
	pub use crate::utils::*;
	pub use crate::{gl_err, gl_ok};
}
