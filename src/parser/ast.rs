
#[derive(Debug)]
pub struct SimpleCommand {
    pub command: String,
    pub args: Vec<String>, 
}

//defines the nodes and edges in AST
#[derive(Debug)]
pub enum AST {
    Simple(SimpleCommand)
}

