use std::io::BufRead;

enum SourceType {
	Content(Vec<char>),
	File(std::io::BufReader<std::fs::File>),
}

pub struct Source {
	chars_cache: Vec<char>,
	data: SourceType,
}

impl Source {
	pub fn new_from_string(content: String) -> std::io::Result<Self> {
		Ok(Self {
			chars_cache: Vec::new(),
			data: SourceType::Content(content.chars().collect()),
		})
	}

	pub fn new_from_filename(filename: String) -> std::io::Result<Self> {
		let f = std::fs::File::open(filename).expect("");
		Ok(Self {
			chars_cache: Vec::new(),
			data: SourceType::File(std::io::BufReader::new(f)),
		})
	}

	pub fn next_char(&mut self) -> Option<char> {
		if self.chars_cache.len() > 0 {
			return Some(self.chars_cache.remove(0));
		}

		match &mut self.data {
			SourceType::Content(content) => {
				if content.len() > 0 {
					self.chars_cache.push(content.remove(0));
				}
			}
			SourceType::File(file) => {
				let mut buf = vec![];
				let _ = file.read_until(b'\n', &mut buf);
				let chars: Vec<char> = String::from_utf8(buf).unwrap().chars().collect();
				for c in chars {
					self.chars_cache.push(c);
				}
				// println!("b {}", self.chars_cache.len());
			}
		}

		if self.chars_cache.len() > 0 {
			Some(self.chars_cache.remove(0))
		} else {
			None
		}
	}
}
