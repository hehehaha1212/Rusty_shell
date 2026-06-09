#[allow(unused_imports)] // flags are written in this format #[...]
use std::io::{self, Write};  

fn main() {
    
    loop{
     print!("$ ");
     io::stdout().flush().unwrap(); //unwrap returns the value if present and error if type or presense is not correct
     let mut command = String:: new();   //makes us a new mutable string variable
     io::stdin().read_line(&mut command).unwrap();
     command=command.trim().to_string();

     if (command.to_lowercase() == "exit") || (command.to_lowercase()== "quit") {
        break;
     }
     else if command.starts_with("echo "){
         println!("{}",&command[5..]);
     }
     else {
         println!("{}: command not found", command.trim());
     }

    
    
    }
   
}
