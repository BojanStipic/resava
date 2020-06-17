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

use ignore::Walk;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

use resava::preprocessors::{AsmPreprocessor, CPreprocessor, Preprocessor, TextPreprocessor};

/// Plagiarism detection for source code
#[derive(StructOpt, Debug)]
#[structopt(set_term_width = 80)]
struct Cli {
    /// Only show files with specified similarity percentage.
    #[structopt(short, long, default_value = "80")]
    similarity: f64,

    /// File preprocessor to use.
    ///
    /// * "asm": x86 GAS assembly
    /// * "c": C programming language
    /// * "text": Basic text preprocessing
    /// * "none": Disable preprocessing {n}
    #[structopt(
        short,
        long,
        default_value = "asm",
        possible_values = &["asm", "c", "text", "none"],
        verbatim_doc_comment,
    )]
    preprocessor: String,

    /// Source file to check for plagiarism.
    source: PathBuf,
    /// Targets to compare against the source file.
    /// If a target is a directory, it is searched recursively.
    #[structopt(default_value = "./")]
    targets: Vec<PathBuf>,
}

fn main() {
    let cli = Cli::from_args();

    // Walk directories recursively
    let targets = walk_directories(&cli.targets);

    // Select preprocessor
    let preprocessor = get_preprocessor(&cli.preprocessor);

    // Detect plagiarism
    for result in resava::detect(&cli.source, &targets, preprocessor.as_deref()) {
        match result {
            Ok((target, score)) if score >= cli.similarity / 100. => {
                println!("\"{}\" : {:.2}%", target.display(), score * 100.);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
            _ => {}
        }
    }
}

fn get_preprocessor(pp: &str) -> Option<Box<dyn Preprocessor + Sync>> {
    match pp {
        "asm" => Some(Box::new(AsmPreprocessor::new())),
        "c" => Some(Box::new(CPreprocessor::new())),
        "text" => Some(Box::new(TextPreprocessor::new())),
        "none" => None,
        _ => unreachable!(),
    }
}

fn walk_directories<P: AsRef<Path>>(paths: &[P]) -> Vec<PathBuf> {
    paths
        .iter()
        .flat_map(|path| {
            Walk::new(path)
                .inspect(|entry| {
                    if let Err(e) = entry {
                        eprintln!("{}", e);
                    }
                })
                .filter_map(Result::ok)
                // Filter: only files; ignore directories
                .filter(|entry| entry.file_type().map_or(false, |e| e.is_file()))
                .map(|entry| entry.into_path())
        })
        .collect()
}
