// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

extern crate gl_core;

use gl_core::preludes::*;

#[test]
fn integer() {
	let add = (Object::Integer(42.to_bigint().unwrap()) + Object::Integer(42.to_bigint().unwrap()))
		.unwrap();
	assert!(add == Object::Integer(84.to_bigint().unwrap()));

	let sub = (Object::Integer(42.to_bigint().unwrap()) - Object::Integer(42.to_bigint().unwrap()))
		.unwrap();
	assert!(sub == Object::Integer(0.to_bigint().unwrap()));

	let mul = (Object::Integer(42.to_bigint().unwrap()) * Object::Integer(42.to_bigint().unwrap()))
		.unwrap();
	assert!(mul == Object::Integer(1764.to_bigint().unwrap()));

	let div = (Object::Integer(42.to_bigint().unwrap()) / Object::Integer(42.to_bigint().unwrap()))
		.unwrap();
	assert!(div == Object::Integer(1.to_bigint().unwrap()));

	let less = Object::Integer(41.to_bigint().unwrap()) < Object::Integer(42.to_bigint().unwrap());
	assert!(less);

	let less_than_equal =
		Object::Integer(42.to_bigint().unwrap()) <= Object::Integer(42.to_bigint().unwrap());
	assert!(less_than_equal);

	let greater =
		Object::Integer(42.to_bigint().unwrap()) > Object::Integer(41.to_bigint().unwrap());
	assert!(greater);

	let greater_than_equal =
		Object::Integer(42.to_bigint().unwrap()) >= Object::Integer(42.to_bigint().unwrap());
	assert!(greater_than_equal);

	let equal =
		Object::Integer(42.to_bigint().unwrap()) == Object::Integer(42.to_bigint().unwrap());
	assert!(equal);

	let not_equal =
		Object::Integer(41.to_bigint().unwrap()) != Object::Integer(42.to_bigint().unwrap());
	assert!(not_equal);
}

#[test]
#[should_panic]
fn integer_divide_by_zero() {
	(Object::Integer(42.to_bigint().unwrap()) / Object::Integer(0.to_bigint().unwrap())).unwrap();
}

#[test]
fn float() {
	let add = (Object::Float(str_to_big_rational("42.5").unwrap())
		+ Object::Float(str_to_big_rational("42.3").unwrap()))
	.unwrap();
	assert!(add == Object::Float(str_to_big_rational("84.8").unwrap()));

	let sub = (Object::Float(str_to_big_rational("42.5").unwrap())
		- Object::Float(str_to_big_rational("42.3").unwrap()))
	.unwrap();
	assert!(sub == Object::Float(str_to_big_rational("0.2").unwrap()));

	let mul = (Object::Float(str_to_big_rational("42.5").unwrap())
		* Object::Float(str_to_big_rational("42.3").unwrap()))
	.unwrap();
	assert!(mul == Object::Float(str_to_big_rational("1797.75").unwrap()));

	let div = (Object::Float(str_to_big_rational("42.5").unwrap())
		/ Object::Float(str_to_big_rational("42.3").unwrap()))
	.unwrap();
	assert!(
		div == Object::Float(
			str_to_big_rational("1.(0047281323877068557919621749408983451536643026)").unwrap()
		)
	);

	let less = Object::Float(str_to_big_rational("42.3").unwrap())
		< Object::Float(str_to_big_rational("42.5").unwrap());
	assert!(less);

	let less_than_equal = Object::Float(str_to_big_rational("42.3").unwrap())
		<= Object::Float(str_to_big_rational("42.3").unwrap());
	assert!(less_than_equal);

	let greater = Object::Float(str_to_big_rational("42.5").unwrap())
		> Object::Float(str_to_big_rational("42.3").unwrap());
	assert!(greater);

	let greater_than_equal = Object::Float(str_to_big_rational("42.5").unwrap())
		>= Object::Float(str_to_big_rational("42.5").unwrap());
	assert!(greater_than_equal);

	let equal = Object::Float(str_to_big_rational("42.5").unwrap())
		== Object::Float(str_to_big_rational("42.5").unwrap());
	assert!(equal);

	let not_equal = Object::Float(str_to_big_rational("42.3").unwrap())
		!= Object::Float(str_to_big_rational("42.5").unwrap());
	assert!(not_equal);
}

#[test]
#[should_panic]
fn float_divide_by_zero() {
	(Object::Float(str_to_big_rational("42.5").unwrap())
		/ Object::Float(str_to_big_rational("0").unwrap()))
	.unwrap();
}

#[test]
fn boolean() {
	let add = (Object::Boolean(true) + Object::Boolean(false)).unwrap();
	assert!(add == Object::Integer(1.to_bigint().unwrap()));

	let sub = (Object::Boolean(true) - Object::Boolean(false)).unwrap();
	assert!(sub == Object::Integer(1.to_bigint().unwrap()));

	let mul = (Object::Boolean(true) * Object::Boolean(false)).unwrap();
	assert!(mul == Object::Integer(0.to_bigint().unwrap()));

	let div = (Object::Boolean(true) / Object::Boolean(true)).unwrap();
	assert!(div == Object::Integer(1.to_bigint().unwrap()));

	let less = Object::Boolean(false) < Object::Boolean(true);
	assert!(less);

	let less_than_equal = Object::Boolean(false) <= Object::Boolean(false);
	assert!(less_than_equal);

	let greater = Object::Boolean(true) > Object::Boolean(false);
	assert!(greater);

	let greater_than_equal = Object::Boolean(true) >= Object::Boolean(true);
	assert!(greater_than_equal);

	let equal = Object::Boolean(true) == Object::Boolean(true);
	assert!(equal);

	let not_equal = Object::Boolean(false) != Object::Boolean(true);
	assert!(not_equal);
}

#[test]
#[should_panic]
fn boolean_divide_by_zero() { (Object::Boolean(true) / Object::Boolean(false)).unwrap(); }
