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

use clap::{Parser, ValueEnum};
use ignore::Walk;
use std::path::{Path, PathBuf};

use resava::preprocessors::{AsmPreprocessor, CPreprocessor, Preprocessor, TextPreprocessor};

/// Plagiarism detection for source code
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Only show files with specified similarity percentage.
    #[arg(short, long, default_value_t = 80.)]
    similarity: f64,

    /// File preprocessor to use.
    #[arg(
        short,
        long,
        default_value_t = PreprocessorArg::Asm,
        value_enum,
    )]
    preprocessor: PreprocessorArg,

    /// Source file to check for plagiarism.
    source: PathBuf,
    /// Targets to compare against the source file.
    /// If a target is a directory, it is searched recursively.
    #[arg(default_value = "./")]
    targets: Vec<PathBuf>,
}

#[derive(ValueEnum, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum PreprocessorArg {
    /// x86 GAS assembly
    Asm,
    /// C programming language
    C,
    /// Basic text preprocessing
    Text,
    /// Disable preprocessing
    None,
}

fn main() {
    let cli = Cli::parse();

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
                eprintln!("{e}");
            }
            _ => {}
        }
    }
}

fn get_preprocessor(pp: &PreprocessorArg) -> Option<Box<dyn Preprocessor + Sync>> {
    match pp {
        PreprocessorArg::Asm => Some(Box::new(AsmPreprocessor::new())),
        PreprocessorArg::C => Some(Box::new(CPreprocessor::new())),
        PreprocessorArg::Text => Some(Box::new(TextPreprocessor::new())),
        PreprocessorArg::None => None,
    }
}

fn walk_directories<P: AsRef<Path>>(paths: &[P]) -> Vec<PathBuf> {
    paths
        .iter()
        .flat_map(|path| {
            Walk::new(path)
                .inspect(|entry| {
                    if let Err(e) = entry {
                        eprintln!("{e}");
                    }
                })
                .filter_map(Result::ok)
                .filter(|entry| entry.file_type().is_some_and(|e| e.is_file()))
                .map(|entry| entry.into_path())
        })
        .collect()
}
