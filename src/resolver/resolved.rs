use std::path::PathBuf;

#[derive(Debug,Clone)]
pub enum ResolvedCommand {
    SpecialBuiltin(CommandInvocation),
    Builtin(CommandInvocation),
    Function(CommandInvocation),
    External {
        path: PathBuf,
        invocation: CommandInvocation,
    },
}
#[derive(Debug,Clone)]
pub struct CommandInvocation {
    pub name: String,
    pub args: Vec<String>,
}