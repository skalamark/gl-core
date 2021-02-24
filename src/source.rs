// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use std::fs::File;
use std::io::{self, BufReader, Read};

enum SourceType {
	String(Vec<char>, String),
	File(BufReader<File>),
}

pub struct Source {
	chars_cache: Vec<char>,
	data: SourceType,
}

impl Source {
	pub fn from_strig(string: String) -> io::Result<Self> {
		Ok(Self {
			chars_cache: Vec::new(),
			data: SourceType::String(Vec::new(), string),
		})
	}

	pub fn from_file(filename: String) -> io::Result<Self> {
		let file: File = File::open(filename)?;
		Ok(Self {
			chars_cache: Vec::new(),
			data: SourceType::File(BufReader::new(file)),
		})
	}

	pub fn next_char(&mut self) -> Option<char> {
		if self.chars_cache.len() > 0 {
			return Some(self.chars_cache.remove(0));
		}

		match &mut self.data {
			SourceType::String(chars, string) => {
				if chars.len() > 0 {
					for _ in 0..10 {
						if chars.len() > 0 {
							self.chars_cache.push(chars.remove(0));
						}
					}
				} else if string.len() > 0 {
					let mut limite_read_chars: usize = 10;
					loop {
						let read_chars: Vec<char> = match string.get(0..limite_read_chars) {
							Some(read_chars) => read_chars.chars().collect(),
							None => {
								limite_read_chars += 1;
								continue;
							}
						};
						for one_char in read_chars {
							chars.push(one_char);
						}
						break;
					}
					return self.next_char();
				}
			}
			SourceType::File(file) => {
				let mut buffer: [u8; 10] = [0; 10];
				let mut buffer_vec: Vec<u8> = Vec::new();
				loop {
					if file.read_exact(&mut buffer).is_err() {
						buffer_vec.extend(buffer.to_vec());
					}
					let string_buffer: String = String::from_utf8(buffer_vec).unwrap();
					for one_char in string_buffer.chars() {
						self.chars_cache.push(one_char);
					}
					break;
				}
			}
		}

		if self.chars_cache.len() > 0 {
			return self.next_char();
		} else {
			None
		}
	}
}
