#[allow(unused_imports)] // flags are written in this format #[...]
use std::io::{self, Write};  

mod shell;
mod lexer;

use shell::Shell;

fn main() {
    let mut shell = Shell::new();

    shell.run();
}