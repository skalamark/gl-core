// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

extern crate gl_core;

use std::ops::Div;

use num::{BigInt, BigRational, ToPrimitive};

#[test]
fn to_u8() {
	let bigint = BigInt::parse_bytes(b"10", 10).unwrap();
	assert_eq!(10u8, bigint.to_u8().unwrap())
}

#[test]
fn to_rational() {
	let bigrational = BigRational::from_float(3.1415).unwrap();
	println!("{:#?}", &bigrational);
	println!(
		"{:#?}",
		bigrational
			.numer()
			.to_f64()
			.unwrap()
			.div(bigrational.denom().to_f64().unwrap())
			.to_f64()
			.unwrap()
	);
	// assert_eq!(10u8, bigint.to_u8().unwrap())
}
