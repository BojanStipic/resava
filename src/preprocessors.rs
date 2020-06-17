// resava - Plagiarism detection for source code
// Copyright (C) 2020 Bojan Stipic
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use regex::Regex;

pub trait Preprocessor {
    fn process(&self, input: &str) -> String;
}

/// Basic text preprocessor.
///
/// * Whitespace normalization
/// * Character case normalization
pub struct TextPreprocessor {
    whitespace: Regex,
}

impl TextPreprocessor {
    pub fn new() -> Self {
        Self {
            whitespace: Regex::new(r"\s+").unwrap(),
        }
    }
}

impl Preprocessor for TextPreprocessor {
    fn process(&self, input: &str) -> String {
        self.whitespace.replace_all(input, " ").to_lowercase()
    }
}

/// x86 GAS assembly preprocessor.
///
/// * Removes comments
/// * … Everything else that basic TextPreprocessor does
pub struct AsmPreprocessor {
    comment: Regex,
    text_preprocessor: TextPreprocessor,
}

impl AsmPreprocessor {
    pub fn new() -> Self {
        Self {
            comment: Regex::new(r"(?m)#.*$").unwrap(),
            text_preprocessor: TextPreprocessor::new(),
        }
    }
}

impl Preprocessor for AsmPreprocessor {
    fn process(&self, input: &str) -> String {
        let input = self.comment.replace_all(input, "");
        self.text_preprocessor.process(&input)
    }
}

/// C language preprocessor.
///
/// * Removes comments
/// * … Everything else that the basic TextPreprocessor does
pub struct CPreprocessor {
    line_comment: Regex,
    multi_comment: Regex,
    text_preprocessor: TextPreprocessor,
}

impl CPreprocessor {
    pub fn new() -> Self {
        Self {
            line_comment: Regex::new(r"(?m)//.*$").unwrap(),
            multi_comment: Regex::new(r"(?s)/\*.*?\*/").unwrap(),
            text_preprocessor: TextPreprocessor::new(),
        }
    }
}

impl Preprocessor for CPreprocessor {
    fn process(&self, input: &str) -> String {
        let input = self.line_comment.replace_all(input, "");
        let input = self.multi_comment.replace_all(&input, "");
        self.text_preprocessor.process(&input)
    }
}
