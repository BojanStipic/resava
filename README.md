# resava

Plagiarism detection for source code.

## Compiling from source

### Prerequisites

* Rust compiler

### Compile and install with Cargo:

```bash
cargo build --release
cargo install --path .
```

## Download precompiled binary

Alternatively, precompiled binaries are available on [Release](https://github.com/BojanStipic/resava/releases) page.

## Usage

```bash
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

            Possible values:
             * "asm": x86 GAS assembly
             * "text": Basic text preprocessing
             * "none": Disable preprocessing
             [default: asm]
    -s, --similarity <similarity>
            Only show files with specified similarity percentage [default: 80]


ARGS:
    <source>
            Source file to check for plagiarism

    <targets>...
            Targets to compare against source file
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
