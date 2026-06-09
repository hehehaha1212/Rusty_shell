#[allow(unused_imports)] // flags are written in this format #[...]
use std::io::{self, Write};  

fn main() {
    
    loop{
     print!("$ ");
     io::stdout().flush().unwrap(); //unwrap returns the value if present and error if type or presense is not correct
     let mut command = String:: new();   //makes us a new mutable string variable
     io::stdin().read_line(&mut command).unwrap();
     println!("{}: command not found", command.trim());
    
    }
   
}
