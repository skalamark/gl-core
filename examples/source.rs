// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

extern crate gl_core;

use gl_core::source::Source;

fn main() {
	let expected_content: String = std::fs::read_to_string("examples/source.rs").unwrap();
	let mut content: String = String::new();
	let mut source: Source = Source::from_filename("examples/source.rs").unwrap();

	loop {
		match source.next_char() {
			Some(ch) => content.push(ch),
			None => break,
		}
	}

	assert_eq!(expected_content, content);
}
