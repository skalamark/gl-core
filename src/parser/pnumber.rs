// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

impl Parser {
	pub fn parse_integer(&mut self, integer_literal: String) -> Result<Literal, Exception> {
		Ok(Literal::Integer(
			BigInt::parse_bytes(integer_literal.as_bytes(), 10).unwrap(),
		))
	}

	pub fn parse_float(&mut self, float_literal: String) -> Result<Literal, Exception> {
		let splits: Vec<&str> = float_literal.split('.').collect::<Vec<&str>>();
		let numer: BigInt = BigInt::parse_bytes(splits[0].as_bytes(), 10).unwrap();
		let denom: BigInt = BigInt::parse_bytes(splits[1].as_bytes(), 10).unwrap();
		Ok(Literal::Float(BigRational::new(numer, denom)))
	}
}
