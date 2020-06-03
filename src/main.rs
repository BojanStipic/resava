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

use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

use resava::preprocessors::{AsmPreprocessor, Preprocessor, TextPreprocessor};
use resava::Resava;

/// Plagiarism detection for source code
#[derive(StructOpt, Debug)]
struct Cli {
    /// Only show files with specified similarity percentage.
    #[structopt(short, long, default_value = "80")]
    similarity: f64,

    /// File preprocessor to use.
    ///
    /// Possible values: {n}
    /// * "asm": x86 GAS assembly {n}
    /// * "text": Basic text preprocessing {n}
    /// * "none": Disable preprocessing {n}
    #[structopt(short, long, default_value = "asm")]
    preprocessor: String,

    /// Source file to check for plagiarism.
    source: PathBuf,
    /// Targets to compare against source file.
    targets: Vec<PathBuf>,
}

fn main() {
    let cli = Cli::from_args();

    // Select preprocessor
    let preprocessor: Option<Box<dyn Preprocessor>> = match cli.preprocessor.as_ref() {
        "asm" => Some(Box::new(AsmPreprocessor::new())),
        "text" => Some(Box::new(TextPreprocessor::new())),
        "none" => None,
        other => {
            eprintln!("\"{}\" is not a valid value for preprocessor", other);
            process::exit(1);
        }
    };

    let resava = Resava::new(cli.source, cli.targets, cli.similarity / 100.);

    for result in &resava.run(preprocessor.as_deref()) {
        match result {
            Ok((target, score)) => {
                println!("\"{}\" : {:.2}%", target.display(), score * 100.);
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }
}
