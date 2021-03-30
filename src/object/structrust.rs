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
