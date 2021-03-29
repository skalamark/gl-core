// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;

use num::{BigInt, BigRational, FromPrimitive, One, Signed, Zero};
use regex::Regex;

lazy_static::lazy_static! {
	static ref TEN: BigInt = BigInt::from_u32(10).unwrap();
	static ref NINE: BigInt = BigInt::from_u32(9).unwrap();
	static ref REGEX: Regex = Regex::new(r"^(?P<neg>-)?(?P<int>\d+)(\.(?P<frac>\d*)(\((?P<repeat>\d+)\))?)?$").unwrap();
}

pub fn big_rational_to_string(mut n: BigRational) -> String {
	let neg: bool = if n.is_negative() {
		n = -n;
		true
	} else {
		false
	};
	let mut fraction: String = String::new();
	let mut map: HashMap<BigInt, usize> = HashMap::new();
	let mut rem: BigInt = n.numer() % n.denom();
	while !rem.is_zero() && !map.contains_key(&rem) {
		map.insert(rem.clone(), fraction.len());
		rem *= TEN.clone();
		fraction.push_str(&(rem.clone() / n.denom()).to_string());
		rem = rem % n.denom();
	}
	let mut output: String = if neg { "-".to_owned() } else { String::new() };
	output.push_str(&(n.numer() / n.denom()).to_string());
	if rem.is_zero() {
		if fraction.len() != 0 {
			let _ = write!(output, ".{}", &fraction);
		}
	} else {
		let _ = write!(output, ".{}({})", &fraction[..map[&rem]], &fraction[map[&rem]..]);
	}
	output
}

pub fn str_to_big_rational(string: &str) -> Result<BigRational, ()> {
	match REGEX.captures(string) {
		Some(captures) => {
			let neg: bool = captures.name("neg").is_some();
			let int: &str = &captures["int"];
			let fraction: Option<&str> = captures.name("frac").map(|a| a.as_str());
			let repeating: Option<&str> = captures.name("repeat").map(|a| a.as_str());

			Ok(match fraction {
				None => {
					let int: BigRational = BigRational::from_str(int).unwrap();
					if neg {
						-int
					} else {
						int
					}
				},
				Some(fraction) => match repeating {
					None => frac(neg, int, fraction),
					Some(repeating) => repeat(neg, int, fraction, repeating),
				},
			})
		},
		None => Err(()),
	}
}

fn frac(neg: bool, integer: &str, fractional: &str) -> BigRational {
	let mut a: BigInt = BigInt::one();
	for _ in 0..fractional.len() {
		a *= &*TEN;
	}
	let b: BigRational = BigRational::new(BigInt::from_str(fractional).unwrap(), a)
		+ BigInt::from_str(integer).unwrap();
	if neg {
		-b
	} else {
		b
	}
}

fn repeat(neg: bool, integer: &str, fractional: &str, repeating: &str) -> BigRational {
	let mut a: BigRational = BigRational::one();
	for _ in 0..fractional.len() {
		a *= &*TEN;
	}
	let b: BigRational = match BigRational::from_str(fractional) {
		Ok(a) => a,
		Err(_) => BigRational::zero(),
	} / &a;
	let mut c: BigInt = NINE.clone();
	for _ in 1..repeating.len() {
		c *= &*TEN;
		c += &*NINE;
	}
	let d: BigRational = BigRational::from_str(repeating).unwrap() / c / a;
	let e: BigRational = b + d + BigInt::from_str(integer).unwrap();
	if neg {
		-e
	} else {
		e
	}
}
