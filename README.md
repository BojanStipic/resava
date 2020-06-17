# resava

Plagiarism detection for source code.

## Plagiarism detection

Plagiarism is detected using the following steps:

1. Input preprocessing:
    * Whitespace normalization
    * Letter case normalization
    * Language dependent preprocessing. Currently supported languages:
        * x86 GAS assembly
        * C programming language
2. Similarity check using string metric algorithm. Currently supported algorithms:
    * Normalized [Levenshtein distance](https://en.wikipedia.org/wiki/Levenshtein_distance)

## Installation options

### Download precompiled binaries

Precompiled binaries are available on [Releases](https://github.com/BojanStipic/resava/releases) page.
Currently only `x86_64-unknown-linux-gnu` targets are supported.

### Compiling from source

#### Prerequisites

* [Rust language toolchain](https://www.rust-lang.org/tools/install)

#### Compile and install with Cargo:

```bash
cargo build --release
cargo install --path .
```

## Usage

```
USAGE:
    resava [OPTIONS] <source> [targets]...

FLAGS:
    -h, --help
            Prints help information

    -V, --version
            Prints version information


OPTIONS:
    -p, --preprocessor <preprocessor>
            File preprocessor to use.

            * "asm": x86 GAS assembly
            * "c": C programming language
            * "text": Basic text preprocessing
            * "none": Disable preprocessing
             [default: asm]  [possible values: asm, c, text, none]
    -s, --similarity <similarity>
            Only show files with specified similarity percentage [default: 80]


ARGS:
    <source>
            Source file to check for plagiarism

    <targets>...
            Targets to compare against the source file. If a target is a
            directory, it is searched recursively [default: ./]
```

## License

    Copyright (C) 2020 Bojan Stipic

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
