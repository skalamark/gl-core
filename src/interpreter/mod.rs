// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

pub struct Interpreter {
	pub(crate) scopes: ScopeStack,
	pub(crate) current_level: usize,
}

#[derive(Clone, PartialEq)]
pub struct ScopeStack {
	global: Scope,
	pub(crate) stack: Vec<Scope>,
}

#[derive(Clone, PartialEq)]
pub struct Scope {
	pub(crate) store: HashMap<String, Object>,
}

// Interpreter

impl Interpreter {
	pub fn new() -> Self { Self { scopes: ScopeStack::new(), current_level: 0 } }

	pub fn eval<T: Into<String>>(&mut self, source: T) -> GLResult {
		let mut result: Object = Object::Null;
		self.scopes.create_scope();
		self.current_level += 1;

		let filename: String = "eval".to_string();
		let source: Source = Source::from_string(source);
		let lexer: Lexer = Lexer::new(source, &filename);
		let mut parser: Parser = Parser::new(lexer)?;

		loop {
			match parser.next()? {
				Some(statement) => result = statement.eval(self)?,
				None => break,
			}
		}

		self.current_level -= 1;

		gl_ok!(result)
	}

	pub fn script<T: Into<String>>(&mut self, filename: T) -> GLResult {
		let mut result: Object = Object::Null;
		self.scopes.create_scope();
		self.current_level += 1;

		let filename: String = filename.into();
		let source: Source = Source::from_filename(&filename).unwrap();
		let lexer: Lexer = Lexer::new(source, &filename);
		let mut parser: Parser = Parser::new(lexer)?;

		loop {
			match parser.next()? {
				Some(statement) => result = statement.eval(self)?,
				None => break,
			}
		}

		self.current_level -= 1;

		gl_ok!(result)
	}

	pub fn get<T: Into<String>>(&self, identifier: T) -> Option<Object> {
		let identifier: String = identifier.into();
		let mut cl: usize = self.current_level;

		while cl > 0 {
			let result = self.scopes.stack[cl].get(identifier.clone());

			if result.is_some() {
				return result;
			}

			cl -= 1;
		}

		match self.scopes.global.get(identifier.clone()) {
			Some(o) => Some(o.clone()),
			None => None,
		}
	}

	pub fn set<T: Into<String>, O: Into<Object>>(&mut self, name: T, value: O) {
		self.scopes.stack[self.current_level].set(name.into(), value.into());
	}

	pub fn set_global<T: Into<String>, O: Into<Object>>(&mut self, name: T, value: O) {
		self.scopes.global.set(name.into(), value.into());
	}
}

// ScopeStack

impl ScopeStack {
	pub fn new() -> Self {
		let mut ss = Self { global: Scope::new(), stack: Vec::new() };
		ss.create_scope();
		ss
	}

	pub fn push_scope(&mut self, scope: Scope) { self.stack.push(scope) }

	pub fn create_scope(&mut self) { self.stack.push(Scope::new()) }
}

// Scope

impl Scope {
	pub fn new() -> Self { Self { store: HashMap::new() } }

	pub fn get<T: Into<String>>(&self, identifier: T) -> Option<Object> {
		let identifier: String = identifier.into();
		match self.store.get(&identifier) {
			Some(o) => Some(o.clone()),
			None => None,
		}
	}

	pub fn set<T: Into<String>>(&mut self, name: T, value: Object) {
		self.store.insert(name.into(), value);
	}
}
