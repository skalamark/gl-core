// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use downcast_rs::{impl_downcast, DowncastSync};
use dyn_clone::{clone_trait_object, DynClone};

use crate::preludes::*;

impl_downcast!(sync ObjectTrait);
clone_trait_object!(ObjectTrait);

pub trait ObjectTrait: std::fmt::Display + std::fmt::Debug + DynClone + DowncastSync {
	fn is_equals(&self, other: &Box<dyn ObjectTrait>) -> bool;
	fn get_env(&self) -> HashMap<String, FNStructRust> { HashMap::new() }
}

impl PartialEq for Box<dyn ObjectTrait> {
	fn eq(&self, other: &Self) -> bool { self.is_equals(other) }
}

#[derive(Debug)]
pub struct StructRust {
	pub o: Box<dyn ObjectTrait>,
	pub env: HashMap<String, FNStructRust>,
}

impl StructRust {
	pub fn new(o: Box<dyn ObjectTrait>) -> Self {
		let env: HashMap<String, FNStructRust> = o.get_env();
		Self { o, env }
	}
}

impl Clone for StructRust {
	fn clone(&self) -> Self { Self::new(self.o.clone()) }
}

impl PartialEq for StructRust {
	fn eq(&self, other: &Self) -> bool { self.o == other.o.clone() }
}

impl Eq for StructRust {}
