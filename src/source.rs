// Copyright 2021 the GLanguage authors. All rights reserved. MIT license.

use std::fs::File;
use std::io::{BufRead, BufReader};

enum SourceType {
	String(Vec<char>),
	File(BufReader<File>),
}
