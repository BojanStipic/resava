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

pub mod preprocessors;

use std::fs;
use std::path::{Path, PathBuf};

use preprocessors::Preprocessor;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("\"{0}\": IO error: {1}")]
    IoError(PathBuf, std::io::Error),
}
pub type Result<T> = std::result::Result<T, Error>;

/// Compare `source` to all `targets` and return target with their similarity score
pub fn detect<P1: AsRef<Path> + Eq, P2: AsRef<Path> + Eq, PP: Preprocessor + ?Sized>(
    source: P1,
    targets: &[P2],
    preprocessor: Option<&PP>,
) -> Vec<Result<(PathBuf, f64)>> {
    let source_content = match parse_content(&source, preprocessor) {
        Ok(content) => content,
        Err(e) => {
            return vec![Err(e)];
        }
    };

    let targets: Vec<&Path> = targets.iter().map(AsRef::as_ref).collect();
    targets
        .into_iter()
        .map(|target| parse_content(target, preprocessor).map(|content| (target, content)))
        .map(|result| {
            result.map(|(target, target_content)| {
                (
                    target.to_owned(),
                    similarity(&source_content, &target_content),
                )
            })
        })
        .collect()
}

fn similarity(a: &str, b: &str) -> f64 {
    strsim::normalized_levenshtein(a, b)
}

fn parse_content<P: AsRef<Path>, PP: Preprocessor + ?Sized>(
    path: P,
    preprocessor: Option<&PP>,
) -> Result<String> {
    match fs::read_to_string(&path) {
        Ok(content) => match preprocessor {
            Some(pp) => Ok(pp.process(&content)),
            None => Ok(content),
        },
        Err(e) => {
            return Err(Error::IoError(path.as_ref().to_owned(), e));
        }
    }
}
