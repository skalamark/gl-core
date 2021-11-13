// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

#[macro_export]
macro_rules! gl_ok {
    () => (
        Ok(Object::Null)
    );
    ($arg:expr) => (
        Ok(Object::from($arg))
    );
    ($($arg:tt)*) => (
        Ok(Object::from(format!($($arg)*)))
    )
}

#[macro_export]
macro_rules! gl_err {
	($arg:expr) => {
		Err($arg)
	};
}
