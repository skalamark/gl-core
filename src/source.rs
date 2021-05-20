// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use std::fs::File;
use std::io::{self, BufReader, Read};

enum SourceType {
	String(String),
	File(BufReader<File>),
}

pub struct Source {
	chars_cache: Vec<char>,
	data: SourceType,
	eof: bool,
}

impl Source {
	pub fn from_string<T: Into<String>>(string: T) -> Self {
		Self { chars_cache: Vec::new(), data: SourceType::String(string.into()), eof: false }
	}

	pub fn from_filename<T: Into<String>>(filename: T) -> io::Result<Self> {
		let file: File = File::open(filename.into())?;

		Ok(Self {
			chars_cache: Vec::new(),
			data: SourceType::File(BufReader::new(file)),
			eof: false,
		})
	}

	pub fn next_char(&mut self) -> Option<char> {
		if self.chars_cache.len() > 0 {
			let chc: char = self.chars_cache.remove(0);
			return Some(chc);
		}

		match &mut self.data {
			SourceType::String(string) =>
				if !self.eof && string.len() > 0 {
					let mut limite_read_chars: usize = 32;

					if string.len() < limite_read_chars {
						limite_read_chars = string.len();
					}

					loop {
						let string_buffer: String =
							if let Some(fstr) = string.get(0..limite_read_chars) {
								format!("{}", fstr)
							} else {
								limite_read_chars += 1;
								continue;
							};

						{
							let fstring: String =
								format!("{}", string.get(limite_read_chars..).unwrap());
							string.clear();
							string.push_str(&fstring);
						}

						self.chars_cache.extend(string_buffer.chars());
						break;
					}
				},
			SourceType::File(file) =>
				if !self.eof {
					let mut buffer: [u8; 256] = [0; 256];
					let n: usize = file.read(&mut buffer).unwrap();

					if let Ok(string_buffer) = String::from_utf8(buffer[..n].to_vec()) {
						for ch in string_buffer.chars() {
							self.chars_cache.push(ch);
						}
					}
				},
		}

		if self.chars_cache.len() > 0 {
			return self.next_char();
		} else {
			None
		}
	}
}
