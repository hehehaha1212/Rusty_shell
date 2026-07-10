use std::process::Command;
use std::path::PathBuf;

use crate::resolver::{ ResolvedCommand};
use crate::resolver::resolved::CommandInvocation;

pub struct Executor;

impl Executor {
    pub fn new() -> Self {
        Self
    }
    pub fn execute(&self, command: ResolvedCommand) -> Result<(), String> {
        match command {
            ResolvedCommand::SpecialBuiltin(invocation) => self.execute_special_builtin(invocation),

            ResolvedCommand::Builtin(invocation) => self.execute_builtin(invocation),

            ResolvedCommand::Function(invocation) => self.execute_function(invocation),

            ResolvedCommand::External { path, invocation } => {
                self.execute_external(path, invocation)
            }
        }
    }
    fn execute_special_builtin(&self, invocation: CommandInvocation) -> Result<(), String> {
        match invocation.name.as_str() {
            "exit" => {
                std::process::exit(0);
            }

            _ => Err(format!(
                "unimplemented special builtin: {}",
                invocation.name
            )),
        }
    }
    fn execute_builtin(&self, invocation: CommandInvocation) -> Result<(), String> {
        match invocation.name.as_str() {
            "echo" => {
                println!("{}", invocation.args.join(" "));

                Ok(())
            }

            "pwd" => {
                let cwd = std::env::current_dir().map_err(|e| e.to_string())?;

                println!("{}", cwd.display());

                Ok(())
            }

            _ => Err(format!("unimplemented builtin: {}", invocation.name)),
        }
    }
    fn execute_function(&self, invocation: CommandInvocation) -> Result<(), String> {
        Err(format!("function not implemented: {}", invocation.name))
    }
    fn execute_external(&self, path: PathBuf, invocation: CommandInvocation) -> Result<(), String> {
        let status = Command::new(path)
            .args(&invocation.args)
            .status()
            .map_err(|e| e.to_string())?;

        if !status.success() {
            return Err(format!("process exited with status {}", status));
        }

        Ok(())
    }
}
