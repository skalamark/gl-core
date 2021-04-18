// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use downcast_rs::{impl_downcast, Downcast};

use crate::preludes::*;

impl_downcast!(ObjectTrait);

pub trait ObjectTrait: Downcast {
	fn is_equals(&self, other: &Box<dyn ObjectTrait>) -> bool;
	fn display(&self) -> String { format!("tst") }
	fn clone(&self) -> Box<dyn ObjectTrait>;
}

impl PartialEq for Box<dyn ObjectTrait> {
	fn eq(&self, other: &Self) -> bool { self.is_equals(other) }
}

pub struct StructRust {
	pub o: Box<dyn ObjectTrait>,
	pub env: HashMap<String, FNStructRust>,
}

impl StructRust {
	pub fn new(o: Box<dyn ObjectTrait>) -> Self {
		let env: HashMap<String, FNStructRust> = HashMap::new();
		Self { o, env }
	}

	pub fn add_function(&mut self, name: String, value: FNStructRust) {
		self.env.insert(name, value);
	}
}

impl Clone for StructRust {
	fn clone(&self) -> Self {
		let mut s = Self::new(self.o.clone());
		s.env = self.env.clone();
		s
	}
}

impl std::fmt::Display for StructRust {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
		write!(f, "{}", self.o.display())
	}
}

impl PartialEq for StructRust {
	fn eq(&self, other: &Self) -> bool { &self.o == &other.o }
}

impl Eq for StructRust {}
