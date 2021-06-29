[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

### Description
This is a disassembler for N64 MIPS code (not necessarily only game roms). Functionality is hardcoded at the moment, so this isn't end-user friendly. The software also has some search functions available for locating certain kinds of instructions.

### Building
Rust is highly integrated with the `cargo` build system. To install Rust and `cargo`, just follow [these instructions](https://doc.rust-lang.org/cargo/getting-started/installation.html). Once installed, while in the project directory, just run `cargo build --release` or to run directly, you can use `cargo run`.
