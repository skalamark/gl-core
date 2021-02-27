// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn parse_integer(&mut self, integer_literal: String) -> Result<Literal, Exception> {
		Ok(Literal::Integer(
			BigInt::parse_bytes(integer_literal.as_bytes(), 10).unwrap(),
		))
	}

	pub fn parse_float(&mut self, float_literal: String) -> Result<Literal, Exception> {
		Ok(Literal::Float(str_to_big_rational(float_literal.as_str()).unwrap()))
	}
}
