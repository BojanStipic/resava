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

#[derive(Debug)]
pub struct Resava {
    source: PathBuf,
    targets: Vec<PathBuf>,
    similarity: f64,
}

impl Resava {
    pub fn new(source: PathBuf, targets: Vec<PathBuf>, similarity: f64) -> Self {
        Self {
            source,
            targets,
            similarity,
        }
    }

    pub fn run<P: Preprocessor + ?Sized>(
        &self,
        preprocessor: Option<&P>,
    ) -> Vec<Result<(&Path, f64)>> {
        let mut results = Vec::new();

        let source_content = match parse_file(&self.source, preprocessor) {
            Ok(content) => content,
            Err(e) => {
                results.push(Err(e));
                return results;
            }
        };

        for target in &self.targets {
            // Don't compare source with itself
            if target == &self.source {
                continue;
            }

            let target_content = match parse_file(&target, preprocessor) {
                Ok(content) => content,
                Err(e) => {
                    results.push(Err(e));
                    continue;
                }
            };

            let score = strsim::normalized_levenshtein(&source_content, &target_content);
            if score >= self.similarity {
                results.push(Ok((target.as_path(), score)));
            }
        }

        results
    }
}

fn parse_file<T: AsRef<Path>, U: Preprocessor + ?Sized>(
    path: T,
    preprocessor: Option<&U>,
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
