// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use crate::preludes::*;

#[derive(Clone, Debug, PartialEq)]
pub struct AbstractSyntaxTree {
	pub statements: Vec<Statement>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Block(pub Vec<Statement>);

#[derive(Clone, Debug, PartialEq)]
pub enum Statement {
	Let(String, Expression),
	LetAlter(String, Expression),
	Expression(Expression),
	ExpressionReturn(Expression),
	Fn { name: String, params: Vec<String>, body: Block },
	Import(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
	Identifier(String),
	Literal(Literal),
	Prefix(Prefix, Box<Expression>),
	Infix(Infix, Box<Expression>, Box<Expression>),
	Fn { params: Vec<String>, body: Block },
	Call { function: Box<Expression>, arguments: Vec<Expression> },
	Index(Box<Expression>, Box<Expression>),
	Property(Box<Expression>, Box<Expression>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
	Null,
	Integer(BigInt),
	Float(BigRational),
	Boolean(bool),
	String(String),
	Vec(Vec<Expression>),
	Tuple(Vec<Expression>),
	HashMap(Vec<(Expression, Expression)>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Prefix {
	Not,   // !
	Plus,  // +
	Minus, // -
}

#[derive(Clone, Debug, PartialEq)]
pub enum Infix {
	Plus,             // +
	Minus,            // -
	Multiply,         // *
	Divide,           // /
	Equal,            // ==
	NotEqual,         // !=
	LessThanEqual,    // <=
	LessThan,         // <
	GreaterThanEqual, // >=
	GreaterThan,      // >
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Precedence {
	Lowest,
	Comma,          // ,
	Equality,       // == or !=
	Relational,     // > or < or >= or <=
	Additive,       // + or -
	Multiplicative, // / or *
	Call,           // function(x)
	Index,          // vec[index] or hashmap[key]
	Property,       // X::Y or X.Y
}

impl std::fmt::Display for Prefix {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Prefix::Not => write!(f, "!"),
			Prefix::Plus => write!(f, "+"),
			Prefix::Minus => write!(f, "-"),
		}
	}
}

impl std::fmt::Display for Infix {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		match self {
			Infix::Plus => write!(f, "+"),
			Infix::Minus => write!(f, "-"),
			Infix::Multiply => write!(f, "*"),
			Infix::Divide => write!(f, "/"),
			Infix::Equal => write!(f, "=="),
			Infix::NotEqual => write!(f, "!="),
			Infix::LessThanEqual => write!(f, "<="),
			Infix::LessThan => write!(f, "<"),
			Infix::GreaterThanEqual => write!(f, ">="),
			Infix::GreaterThan => write!(f, ">"),
		}
	}
}

impl AbstractSyntaxTree {
	pub fn new() -> Self { Self { statements: Vec::new() } }

	pub fn push(&mut self, statement: Statement) { self.statements.push(statement); }
}

impl Precedence {
	pub fn from_token(token: &Token) -> Self { Self::from_token_type(&token.typer) }

	pub fn from_token_type(token_type: &TokenType) -> Self {
		match token_type {
			TokenType::COMMA => Precedence::Comma,
			TokenType::EQUAL | TokenType::NotEqual => Precedence::Equality,
			TokenType::LessThan
			| TokenType::LessThanEqual
			| TokenType::GreaterThan
			| TokenType::GreaterThanEqual => Precedence::Relational,
			TokenType::PLUS | TokenType::MINUS => Precedence::Additive,
			TokenType::ASTERISK | TokenType::SLASH => Precedence::Multiplicative,
			TokenType::LeftParen => Precedence::Call,
			TokenType::LeftBracket => Precedence::Index,
			TokenType::DCOLON => Precedence::Property,
			_ => Precedence::Lowest,
		}
	}
}

// Into Statement

impl Into<Statement> for Expression {
	fn into(self) -> Statement { Statement::Expression(self) }
}

impl Into<Statement> for Literal {
	fn into(self) -> Statement { Statement::Expression(self.into()) }
}

// Into Expression

impl Into<Expression> for Literal {
	fn into(self) -> Expression { Expression::Literal(self) }
}

impl Into<Expression> for BigInt {
	fn into(self) -> Expression { Expression::Literal(self.into()) }
}

impl Into<Expression> for BigRational {
	fn into(self) -> Expression { Expression::Literal(self.into()) }
}

impl Into<Expression> for bool {
	fn into(self) -> Expression { Expression::Literal(self.into()) }
}

impl Into<Expression> for String {
	fn into(self) -> Expression { Expression::Literal(self.into()) }
}

impl Into<Expression> for Vec<Expression> {
	fn into(self) -> Expression { Expression::Literal(self.into()) }
}

impl Into<Expression> for Vec<(Expression, Expression)> {
	fn into(self) -> Expression { Expression::Literal(self.into()) }
}

// Into Literal

impl Into<Literal> for BigInt {
	fn into(self) -> Literal { Literal::Integer(self) }
}

impl Into<Literal> for BigRational {
	fn into(self) -> Literal { Literal::Float(self) }
}

impl Into<Literal> for bool {
	fn into(self) -> Literal { Literal::Boolean(self) }
}

impl Into<Literal> for String {
	fn into(self) -> Literal { Literal::String(self) }
}

impl Into<Literal> for Vec<Expression> {
	fn into(self) -> Literal { Literal::Vec(self) }
}

impl Into<Literal> for Vec<(Expression, Expression)> {
	fn into(self) -> Literal { Literal::HashMap(self) }
}

// Eval

impl Statement {
	pub fn eval(&self, interpreter: &mut Interpreter) -> GLResult {
		match self {
			Statement::Let(name, value) => {
				let value_object: Object = match value.eval(interpreter) {
					Ok(object) => object,
					Err(exception) => {
						interpreter.set(name, Object::Null);
						return Err(exception);
					},
				};

				interpreter.set(name, value_object);
			},
			Statement::LetAlter(name, value) => {
				if !interpreter.scopes.stack[interpreter.current_level].store.contains_key(name) {
					let mut exception: Exception = Exception::in_runtime(Except::name(format!(
						"cannot find value `{}` in this scope",
						name
					)));
					exception.push(ExceptionPoint::new(name, Position::default()));
					return Err(exception);
				}

				let value_object: Object = match value.eval(interpreter) {
					Ok(object) => object,
					Err(exception) => {
						interpreter.set(name, Object::Null);
						return Err(exception);
					},
				};

				interpreter.set(name, value_object);
			},
			Statement::Fn { name, params, body } => {
				interpreter.set(
					name,
					Object::Fn(GFunction::new(Some(name.clone()), params.clone(), body.clone())),
				);
			},
			Statement::Expression(expression) => {
				let _ = expression.eval(interpreter)?;
			},
			Statement::ExpressionReturn(expression) => return expression.eval(interpreter),
			Statement::Import(path_string) => {
				let path = std::path::Path::new(&path_string);
				use libloading::Library;

				if path.is_file() && path.extension().unwrap() == "gl" {
					let _ = interpreter.script(path.to_str().unwrap());
					interpreter.set(
						format!("{}", path.file_stem().unwrap().to_str().unwrap()),
						Object::Module(GModule::new(
							format!("{}", path.file_stem().unwrap().to_str().unwrap()),
							path_string.clone(),
							interpreter
								.scopes
								.stack
								.get(interpreter.current_level + 1)
								.unwrap()
								.clone(),
						)),
					);
				} else {
					let name: String = format!("{}", path.file_stem().unwrap().to_str().unwrap());
					let dynlibrary: Library = unsafe {
						match Library::new(path) {
							Ok(dynlibrary) => dynlibrary,
							Err(err) => {
								let mut exception: Exception =
									Exception::in_runtime(Except::error(err.to_string()));
								exception.push(ExceptionPoint::new(
									"&self.module_context",
									Position::default(),
								));
								return Err(exception);
							},
						}
					};

					let moduledynlibrary: ModuleDynLibrary = ModuleDynLibrary::new(
						&name,
						&path_string,
						Rc::new(RefCell::new(dynlibrary)),
						Rc::new(RefCell::new(Scope::new())),
					);

					if let Ok(Object::FnNative(GFunctionNative { name: _, params_len: _, body })) =
						moduledynlibrary.get_attr("gl_plugin_init")
					{
						body(interpreter, Vec::new())?;
					};

					interpreter.set(name, Object::ModuleDynLibrary(moduledynlibrary));
				}
			},
		}

		gl_ok!(Object::Null)
	}
}

impl Expression {
	pub fn eval(&self, interpreter: &mut Interpreter) -> GLResult {
		let result: Object = match self {
			Expression::Identifier(identifier) => match interpreter.get(identifier) {
				Some(o) => o,
				None => {
					let mut exception: Exception = Exception::in_runtime(Except::name(format!(
						"name '{}' is not defined",
						identifier
					)));
					exception.push(ExceptionPoint::new(
						"self.module_context.clone()",
						Position::default(),
					));
					return Err(exception);
				},
			},
			Expression::Literal(literal) => match literal.clone() {
				Literal::Null => Object::Null,
				Literal::Integer(integer) => Object::Integer(integer),
				Literal::Float(float) => Object::Float(float),
				Literal::Boolean(boolean) => Object::Boolean(boolean),
				Literal::String(string) => Object::String(string),
				Literal::Vec(vector_literal) => {
					let mut vector: Vec<Object> = Vec::new();

					for expression in vector_literal {
						vector.push(expression.eval(interpreter)?);
					}

					Ok(Object::Vec(vector))
				}?,
				Literal::Tuple(tuple_literal) => {
					let mut tuple: Vec<Object> = Vec::new();

					for expression in tuple_literal {
						tuple.push(expression.eval(interpreter)?);
					}

					Ok(Object::Tuple(tuple))
				}?,
				Literal::HashMap(hashmap_literal) => {
					let mut hashmap: HashMap<Object, Object> = HashMap::new();

					for (key_expression, value_expression) in hashmap_literal {
						let key: Object = key_expression.eval(interpreter)?;
						let value: Object = value_expression.eval(interpreter)?;
						hashmap.insert(key, value);
					}

					Ok(Object::HashMap(hashmap))
				}?,
			},
			Expression::Prefix(prefix, right_expression) =>
				self.prefix(prefix.clone(), right_expression.eval(interpreter)?)?,
			Expression::Infix(infix, left_expression, right_expression) => self.infix(
				infix.clone(),
				left_expression.eval(interpreter)?,
				right_expression.eval(interpreter)?,
			)?,
			Expression::Fn { params, body } =>
				Object::Fn(GFunction::new(None, params.clone(), body.clone())),
			Expression::Call { function, arguments } =>
				self.call(interpreter, function.clone(), arguments.clone())?,
			Expression::Index(left_expression, index_expression) =>
				self.index(left_expression.eval(interpreter)?, index_expression.eval(interpreter)?)?,
			Expression::Property(p1, p2) =>
				self.property(interpreter, *p1.clone(), *p2.clone()).0?,
		};

		gl_ok!(result)
	}

	pub fn prefix(&self, prefix: Prefix, right: Object) -> GLResult {
		match prefix {
			Prefix::Not => self.prefix_not_op(right),
			Prefix::Plus => self.prefix_plus_op(right),
			Prefix::Minus => self.prefix_minus_op(right),
		}
	}

	pub fn prefix_not_op(&self, right: Object) -> GLResult {
		Ok(match right {
			Object::Null => Object::Boolean(true),
			Object::Boolean(true) => Object::Boolean(false),
			Object::Boolean(false) => Object::Boolean(true),
			_ => Object::Boolean(false),
		})
	}

	pub fn prefix_plus_op(&self, right: Object) -> GLResult {
		Ok(match right {
			Object::Integer(integer) => Object::Integer(integer),
			Object::Float(float) => Object::Float(float),
			Object::Boolean(boolean) => match boolean {
				true => Object::Integer(1.to_bigint().unwrap()),
				false => Object::Integer(0.to_bigint().unwrap()),
			},
			o => {
				let mut exception: Exception = Exception::in_runtime(Except::type_(format!(
					"bad operand type for unary +: '{}'",
					o.typer()
				)));
				exception
					.push(ExceptionPoint::new("self.module_context.clone(),", Position::default()));
				return Err(exception);
			},
		})
	}

	pub fn prefix_minus_op(&self, right: Object) -> GLResult {
		Ok(match right {
			Object::Integer(integer) => Object::Integer(-integer),
			Object::Float(float) => Object::Float(-float),
			Object::Boolean(boolean) => match boolean {
				true => Object::Integer(-1.to_bigint().unwrap()),
				false => Object::Integer(0.to_bigint().unwrap()),
			},
			o => {
				let mut exception: Exception = Exception::in_runtime(Except::type_(format!(
					"bad operand type for unary -: '{}'",
					o.typer()
				)));
				exception
					.push(ExceptionPoint::new("self.module_context.clone(),", Position::default()));
				return Err(exception);
			},
		})
	}

	pub fn index(&self, left: Object, index: Object) -> GLResult {
		match (left, index) {
			(Object::Vec(vector), Object::Integer(integer)) => self.index_vec(vector, integer),
			(Object::Vec(_), index) => {
				let mut exception: Exception = Exception::in_runtime(Except::type_(format!(
					"list indices must be integers, not {}",
					index.typer()
				)));
				exception
					.push(ExceptionPoint::new("self.module_context.clone(),", Position::default()));
				return Err(exception);
			},
			(Object::HashMap(hashmap), index) => match hashmap.get(&index) {
				Some(object) => Ok(object.clone()),
				None => {
					let mut exception: Exception =
						Exception::in_runtime(Except::key(format!("{}", index)));
					exception.push(ExceptionPoint::new(
						"self.module_context.clone(),",
						Position::default(),
					));
					return Err(exception);
				},
			},
			(o, _) => {
				let mut exception: Exception = Exception::in_runtime(Except::type_(format!(
					"'{}' object is not subscriptable",
					o.typer()
				)));
				exception
					.push(ExceptionPoint::new("self.module_context.clone(),", Position::default()));
				return Err(exception);
			},
		}
	}

	pub fn index_vec(&self, vector: Vec<Object>, index: BigInt) -> GLResult {
		if index < 0.to_bigint().unwrap() {
			let mut exception: Exception =
				Exception::in_runtime(Except::type_(format!("vec index out of range")));
			exception
				.push(ExceptionPoint::new("self.module_context.clone(),", Position::default()));
			return Err(exception);
		}

		match vector.get(index.to_usize().unwrap()) {
			Some(object) => Ok(object.clone()),
			None => {
				let mut exception: Exception =
					Exception::in_runtime(Except::index(format!("vec index out of range")));
				exception
					.push(ExceptionPoint::new("self.module_context.clone(),", Position::default()));
				return Err(exception);
			},
		}
	}

	pub fn infix(&self, infix: Infix, left: Object, right: Object) -> GLResult {
		match {
			match infix {
				Infix::Plus => left + right,
				Infix::Minus => left - right,
				Infix::Multiply => left * right,
				Infix::Divide => left / right,
				Infix::Equal => Ok(Object::Boolean(left == right)),
				Infix::NotEqual => Ok(Object::Boolean(left != right)),
				Infix::LessThan => Ok(Object::Boolean(left < right)),
				Infix::LessThanEqual => Ok(Object::Boolean(left <= right)),
				Infix::GreaterThan => Ok(Object::Boolean(left > right)),
				Infix::GreaterThanEqual => Ok(Object::Boolean(left >= right)),
			}
		} {
			Ok(r) => Ok(r),
			Err(mut exception) => {
				exception
					.push(ExceptionPoint::new("self.module_context.clone(),", Position::default()));
				Err(exception)
			},
		}
	}

	pub fn call(
		&self, interpreter: &mut Interpreter, function: Box<Expression>, arguments: Vec<Expression>,
	) -> GLResult {
		let rf = function.eval(interpreter)?;
		self.call_object(interpreter, rf, arguments)
	}

	pub fn call_object(
		&self, interpreter: &mut Interpreter, function: Object, arguments: Vec<Expression>,
	) -> GLResult {
		let mut result: Object = Object::Null;
		let mut args: Vec<Object> = Vec::new();
		for arg in arguments {
			args.push(arg.eval(interpreter)?);
		}

		let (params, body) = match function {
			Object::FnNative(GFunctionNative { name, params_len, body }) =>
				return if params_len < 0 || params_len == args.len() as i32 {
					match body(interpreter, args) {
						Ok(object) => Ok(object),
						Err(mut exception) => {
							exception.push(ExceptionPoint::new(
								"self.module_context.clone(),",
								Position::default(),
							));
							Err(exception)
						},
					}
				} else {
					let exception: Exception = Exception::in_runtime(Except::type_(format!(
						"{}() expected {} argument, found {}",
						match name {
							Some(name_fn) => name_fn,
							None => format!("<anonymous>"),
						},
						params_len,
						args.len(),
					)));
					Err(exception)
				},
			Object::Fn(GFunction { name, params, body }) =>
				if params.len() == args.len() {
					(params, body)
				} else {
					let exception: Exception = Exception::in_runtime(Except::type_(format!(
						"{}() expected {} argument, found {}",
						match name {
							Some(name_fn) => name_fn,
							None => format!("<anonymous>"),
						},
						params.len(),
						args.len(),
					)));
					return Err(exception);
				},
			o => {
				let mut exception: Exception = Exception::in_runtime(Except::type_(format!(
					"'{}' object is not callable",
					o.typer()
				)));
				exception
					.push(ExceptionPoint::new("self.module_context.clone(),", Position::default()));
				return Err(exception);
			},
		};

		// let mut new_scoped: Env = Env::from_parent(Rc::clone(&self.env));
		// for (name, o) in params.iter().zip(args) {
		// 	new_scoped.set(name, o);
		// }

		interpreter.scopes.create_scope();
		interpreter.current_level += 1;

		for (name, o) in params.iter().zip(args) {
			interpreter.set(name, o);
		}

		for statement in body.0 {
			result = statement.eval(interpreter)?;
		}

		interpreter.current_level -= 1;

		gl_ok!(result)
	}

	fn no_attribute(&self, typer: String, identifier: String) -> GLResult {
		let mut exception: Exception = Exception::in_runtime(Except::attribute(format!(
			"'{}' object has no attribute '{}'",
			typer, identifier
		)));
		exception.push(ExceptionPoint::new("self.module_context.clone()", Position::default()));
		Err(exception)
	}

	fn invalid_syntax(&self) -> GLResult {
		let mut exception: Exception =
			Exception::in_runtime(Except::invalid_syntax(format!("invalid syntax")));
		exception.push(ExceptionPoint::new("self.module_context.clone()", Position::default()));
		Err(exception)
	}

	pub fn property(
		&self, interpreter: &mut Interpreter, from_experssion: Expression, property: Expression,
	) -> (GLResult, String) {
		let from: Object = match from_experssion {
			Expression::Identifier(f) => match interpreter.get(&f) {
				Some(o) => o,
				None =>
					return (
						{
							let mut exception: Exception = Exception::in_runtime(Except::name(
								format!("name '{}' is not defined", f),
							));
							exception.push(ExceptionPoint::new(
								"self.module_context.clone()",
								Position::default(),
							));
							Err(exception)
						},
						format!(""),
					),
			},
			Expression::Property(p1, p2) => {
				let first_p = self.property(interpreter, *p1, *p2);
				match first_p.0 {
					Ok(o) => o,
					Err(e) => {
						return (Err(e), format!(""));
					},
				}
			},
			_ => match from_experssion.eval(interpreter) {
				Ok(expression) => expression,
				exception => {
					return (exception, format!(""));
				},
			},
		};

		self.property_object(interpreter, from, property)
	}

	pub fn property_object(
		&self, interpreter: &mut Interpreter, from: Object, property: Expression,
	) -> (GLResult, String) {
		let from_typer: String = from.typer().to_string();
		match from {
			Object::Module(gm) => self.property_from_module(interpreter, gm, property),
			Object::ModuleNative(gmn) => self.property_from_modulerust(interpreter, gmn, property),
			Object::ModuleDynLibrary(module) =>
				self.property_from_moduledynlibrary(interpreter, module, property),
			Object::StructRust(module) =>
				self.property_from_structrust(interpreter, from_typer, module, property),
			object =>
				if let Expression::Identifier(identifier) = property {
					(self.no_attribute(object.typer().to_string(), identifier), format!(""))
				} else if let Expression::Call { function, arguments: _ } = property {
					if let Expression::Identifier(identifier) = *function {
						(self.no_attribute(object.typer().to_string(), identifier), format!(""))
					} else {
						(self.invalid_syntax(), format!(""))
					}
				} else {
					(self.invalid_syntax(), format!(""))
				},
		}
	}

	fn property_from_module(
		&self, interpreter: &mut Interpreter, gmn: GModule, property: Expression,
	) -> (GLResult, String) {
		if let Expression::Identifier(identifier) = property {
			match gmn.get(identifier.clone()) {
				Some(o) => (Ok(o.clone()), identifier.clone()),
				None => {
					let mut exception: Exception = Exception::in_runtime(Except::attribute(
						format!("module '{}' has no attribute '{}'", gmn.name, identifier),
					));
					exception.push(ExceptionPoint::new(
						"self.module_context.clone()",
						Position::default(),
					));
					(Err(exception), format!(""))
				},
			}
		} else if let Expression::Call { function, arguments } = property {
			if let Expression::Identifier(identifier) = *function {
				match gmn.get(identifier.clone()) {
					Some(o) =>
						(self.call_object(interpreter, o.clone(), arguments), identifier.clone()),
					None => {
						let mut exception: Exception = Exception::in_runtime(Except::attribute(
							format!("module '{}' has no attribute '{}'", gmn.name, identifier),
						));
						exception.push(ExceptionPoint::new(
							"self.module_context.clone()",
							Position::default(),
						));
						(Err(exception), format!(""))
					},
				}
			} else {
				(self.invalid_syntax(), format!(""))
			}
		} else if let Expression::Property(p1, p2) = property {
			match self.property_from_module(interpreter, gmn, *p1).0 {
				Ok(o) => self.property_object(interpreter, o, *p2),
				e => return (e, format!("")),
			}
		} else {
			(self.invalid_syntax(), format!(""))
		}
	}

	fn property_from_modulerust(
		&self, interpreter: &mut Interpreter, gmn: GModuleNative, property: Expression,
	) -> (GLResult, String) {
		if let Expression::Identifier(identifier) = property {
			match gmn.get(identifier.clone()) {
				Some(o) => (Ok(o.clone()), identifier.clone()),
				None => {
					let mut exception: Exception = Exception::in_runtime(Except::attribute(
						format!("module '{}' has no attribute '{}'", gmn.name, identifier),
					));
					exception.push(ExceptionPoint::new(
						"self.module_context.clone()",
						Position::default(),
					));
					(Err(exception), format!(""))
				},
			}
		} else if let Expression::Call { function, arguments } = property {
			if let Expression::Identifier(identifier) = *function {
				match gmn.get(identifier.clone()) {
					Some(o) =>
						(self.call_object(interpreter, o.clone(), arguments), identifier.clone()),
					None => {
						let mut exception: Exception = Exception::in_runtime(Except::attribute(
							format!("module '{}' has no attribute '{}'", gmn.name, identifier),
						));
						exception.push(ExceptionPoint::new(
							"self.module_context.clone()",
							Position::default(),
						));
						(Err(exception), format!(""))
					},
				}
			} else {
				(self.invalid_syntax(), format!(""))
			}
		} else if let Expression::Property(p1, p2) = property {
			match self.property_from_modulerust(interpreter, gmn, *p1).0 {
				Ok(o) => self.property_object(interpreter, o, *p2),
				e => return (e, format!("")),
			}
		} else {
			(self.invalid_syntax(), format!(""))
		}
	}

	fn property_from_moduledynlibrary(
		&self, interpreter: &mut Interpreter, module: ModuleDynLibrary, property: Expression,
	) -> (GLResult, String) {
		if let Expression::Identifier(identifier) = property {
			match module.get_attr(&identifier) {
				Ok(f) => (Ok(f), identifier.clone()),
				Err(_) => {
					let mut exception: Exception = Exception::in_runtime(Except::attribute(
						format!("module '{}' has no attribute '{}'", module.get_name(), identifier),
					));
					exception.push(ExceptionPoint::new(
						"self.module_context.clone()",
						Position::default(),
					));
					(Err(exception), format!(""))
				},
			}
		} else if let Expression::Call { function, arguments } = property {
			if let Expression::Identifier(identifier) = *function {
				match module.get_attr(identifier.clone()) {
					Ok(f) => {
						// let mut args: Vec<Object> = Vec::new();
						// for arg in arguments {
						// 	args.push(match arg.eval(interpreter) {
						// 		Ok(expression) => expression,
						// 		exception => {
						// 			return (exception, format!(""));
						// 		},
						// 	});
						// }
						// (Ok(f) /* f(args) */, identifier.clone());
						(self.call_object(interpreter, f.clone(), arguments), identifier.clone())
					},
					Err(_) => {
						let mut exception: Exception =
							Exception::in_runtime(Except::attribute(format!(
								"module '{}' has no attribute '{}'",
								module.get_name(),
								identifier
							)));
						exception.push(ExceptionPoint::new(
							"self.module_context.clone()",
							Position::default(),
						));
						(Err(exception), format!(""))
					},
				}
			} else {
				(self.invalid_syntax(), format!(""))
			}
		} else {
			(self.invalid_syntax(), format!(""))
		}
	}

	fn property_from_structrust(
		&self, interpreter: &mut Interpreter, typer: String, module: StructRust,
		property: Expression,
	) -> (GLResult, String) {
		if let Expression::Call { function, arguments } = property {
			if let Expression::Identifier(identifier) = *function {
				match module.env.get(&identifier) {
					Some(f) => {
						let mut args: Vec<Object> = Vec::new();
						for arg in arguments {
							args.push(match arg.eval(interpreter) {
								Ok(expression) => expression,
								exception => {
									return (exception, format!(""));
								},
							});
						}
						(f(module.o.clone(), args), identifier.clone())
					},
					None => {
						let mut exception: Exception = Exception::in_runtime(Except::attribute(
							format!("struct '{}' has no attribute '{}'", typer, identifier),
						));
						exception.push(ExceptionPoint::new(
							"self.module_context.clone()",
							Position::default(),
						));
						(Err(exception), format!(""))
					},
				}
			} else {
				(self.invalid_syntax(), format!(""))
			}
		} else {
			(self.invalid_syntax(), format!(""))
		}
	}
}
