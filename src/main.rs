#[allow(unused_imports)] // flags are written in this format #[...]
use std::io::{self, Write};  

fn main() {
    
     print!("$ ");
     io::stdout().flush().unwrap();
}
