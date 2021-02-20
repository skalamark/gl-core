// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

pub struct Function {
	name: Option<String>,
	params: Vec<String>,
	body: Block,
}

impl Object for Function {}

impl std::fmt::Display for Function {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let mut fmt_string: String = String::new();
		fmt_string.push_str(&format!("<function "));
		let name_fn: String = match &self.name {
			Some(name_fn) => name_fn.clone(),
			None => format!("<anonymous>"),
		};
		let mut params_string: String = String::new();
		params_string.push_str(&format!("("));
		for (i, param) in self.params.iter().enumerate() {
			params_string.push_str(&format!("{}", param));
			if i < self.params.len() - 1 {
				params_string.push_str(", ");
			}
		}
		params_string.push_str(&format!(")"));

		fmt_string.push_str(&format!("{} {}>", name_fn, params_string));
		write!(f, "{}", fmt_string)
	}
}

impl std::fmt::Debug for Function {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self)
	}
}

impl Function {
	pub fn new(name: Option<String>, params: Vec<String>, body: Block) -> Self {
		Self { name, params, body }
	}

	pub fn get_name(&self) -> Option<String> {
		self.name.clone()
	}

	pub fn get_params(&self) -> Vec<String> {
		self.params.clone()
	}

	pub fn get_block(&self) -> Block {
		self.body.clone()
	}
}
