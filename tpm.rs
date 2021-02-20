// pub type BuiltinFn = fn(Vec<Objs>, String, Position) -> Result<Objs, ExceptionMain>;
// pub type FnRust = fn(Vec<Objs>, String, Position) -> Result<Objs, ExceptionMain>;

// pub trait ClassTrait: std::fmt::Debug {
// 	fn copy(&self) -> Box<dyn ClassTrait>;
// 	fn too(&self);
// }

// #[derive(Debug)]
// pub struct Class {
// 	pub my_trait: Box<dyn ClassTrait>,
// }

// impl Class {
// 	pub fn copy(&self) -> Self {
// 		Class {
// 			my_trait: self.my_trait.copy(),
// 		}
// 	}
// }

// impl Eq for Class {}
// impl PartialEq for Class {
// 	fn eq(&self, _: &Self) -> bool {
// 		true
// 	}
// }

// #[derive(Debug, PartialEq)]
// pub enum Objs {
// 	BuiltinRust {
// 		name: String,
// 		len_args: i32,
// 		builtinfn: BuiltinFn,
// 	},
// 	FnRust {
// 		name: String,
// 		len_args: i32,
// 		fnrust: FnRust,
// 	},
// 	ClassRust(Class),
// }

// impl std::fmt::Display for Objs {
// 	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
// 		match self {
// 			Objs::Vec(vector) => {
// 			}
// 			Objs::HashMap(hashmap) => {

// 			}
// 			Objs::BuiltinRust {
// 				name,
// 				len_args: _,
// 				builtinfn: _,
// 			} => {
// 				write!(f, "<built-in function {}>", name)
// 			}
// 			Objs::Fn(name, params, _) => {

// 			}
// 			Objs::FnRust {
// 				name,
// 				len_args: _,
// 				fnrust: _,
// 			} => {
// 				write!(f, "<function {}>", name)
// 			}
// 			Objs::ClassRust(_) => {
// 				write!(f, "class")
// 			}
// 		}
// 	}
// }

// impl Eq for Objs {}

// impl Hash for Objs {
// 	fn hash<H: Hasher>(&self, state: &mut H) {
// 		match *self {
// 			Objs::Integer(ref i) => i.integer.hash(state),
// 			Objs::Boolean(ref b) => b.boolean.hash(state),
// 			Objs::String(ref s) => s.hash(state),
// 			_ => "".hash(state),
// 		}
// 	}
// }

// impl Objs {
// 	pub fn typer(&self) -> &str {
// 		match self {
// 			Objs::Null => "null",
// 			Objs::Integer(_) => "integer",
// 			Objs::Boolean(_) => "boolean",
// 			Objs::String(_) => "string",
// 			Objs::Vec(_) => "vec",
// 			Objs::HashMap(_) => "hashmap",
// 			Objs::BuiltinRust {
// 				name: _,
// 				len_args: _,
// 				builtinfn: _,
// 			} => "builtin",
// 			Objs::Fn(_, _, _)
// 			| Objs::FnRust {
// 				name: _,
// 				len_args: _,
// 				fnrust: _,
// 			} => "function",
// 			Objs::ClassRust(_) => "class",
// 		}
// 	}

// 	pub fn copy(&self) -> Objs {
// 		match self {
// 			Objs::Null => Objs::Null,
// 			Objs::Integer(integer) => Objs::Integer(Integer::new(integer.integer.clone())),
// 			Objs::Boolean(boolean) => Objs::Boolean(Boolean::new(boolean.boolean.clone())),
// 			Objs::String(string) => Objs::String(string.clone()),
// 			Objs::Vec(vector) => {
// 				let mut r: Vec<Objs> = vec![];
// 				for Objs in vector.iter() {
// 					r.push(Objs.copy());
// 				}
// 				Objs::Vec(r)
// 			}
// 			Objs::HashMap(hashmap) => {
// 				let mut r: HashMap<Objs, Objs> = HashMap::new();
// 				for (key, value) in hashmap.iter() {
// 					r.insert(key.copy(), value.copy());
// 				}
// 				Objs::HashMap(r)
// 			}
// 			Objs::Fn(name, args, body) => Objs::Fn(name.clone(), args.clone(), body.clone()),
// 			Objs::FnRust {
// 				name,
// 				len_args,
// 				fnrust,
// 			} => Objs::FnRust {
// 				name: name.clone(),
// 				len_args: *len_args,
// 				fnrust: fnrust.clone(),
// 			},
// 			Objs::ClassRust(class) => {
// 				class.my_trait.too();
// 				Objs::ClassRust(class.copy())
// 			}
// 			Objs::BuiltinRust {
// 				name,
// 				len_args,
// 				builtinfn,
// 			} => Objs::BuiltinRust {
// 				name: name.clone(),
// 				len_args: *len_args,
// 				builtinfn: builtinfn.clone(),
// 			},
// 		}
// 	}
// }

mod n {
	// type Method =
	// 	fn(Integer, Vec<Box<dyn Object>>, String, Position) -> Result<Box<dyn Object>, ExceptionMain>;
	// let env: HashMap<String, Method> = this.get_env();
	// this.env = env;
	// this
	// #[derive(Debug, PartialEq)]

	// fn get_env(&self) -> HashMap<String, Method> {
	// 	let mut env: HashMap<String, Method> = HashMap::new();

	// 	env.insert("to_string".to_string(), Self::to_string);
	// 	env.insert("mmove".to_string(), Self::mmove);
	// 	env
	// }

	// pub fn to_string(
	// 	self: Integer, _: Vec<Box<dyn Object>>, _: String, _: Position,
	// ) -> Result<Box<dyn Object>, ExceptionMain> {
	// 	Ok(Object::String(self.integer.to_string()))
	// }

	// pub fn mmove(
	// 	self: Integer, _: Vec<Box<dyn Object>>, _: String, _: Position,
	// ) -> Result<Box<dyn Object>, ExceptionMain> {
	// 	Ok(Object::String(format!(
	// 		"modenfo {} movido",
	// 		self.integer.to_string()
	// 	)))
	// }
}
