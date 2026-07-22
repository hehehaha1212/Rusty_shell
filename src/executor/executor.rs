use crate::resolver::ResolvedCommand;
use crate::resolver::resolved::CommandInvocation;
use crate::state::ShellState;
use std::os::unix::{self, process};
use std::path::PathBuf;
use std::process::Command;

pub struct Executor {
    //previous_dir: Option<PathBuf>,
    //curr_dir: Option<PathBuf>,
}

impl Executor {
    pub fn new() -> Self {
        Self {}
    }
    pub fn execute(&self, state: &mut ShellState, command: ResolvedCommand) -> Result<(), String> {
        match command {
            ResolvedCommand::SpecialBuiltin(invocation) => {
                self.execute_special_builtin(state, invocation)
            }

            ResolvedCommand::Builtin(invocation) => self.execute_builtin(state, invocation),

            ResolvedCommand::Function(invocation) => self.execute_function(invocation),

            ResolvedCommand::External { path, invocation } => {
                self.execute_external(path, invocation)
            }
        }
    }
    fn execute_special_builtin(
        &self,
        state: &mut ShellState,
        invocation: CommandInvocation,
    ) -> Result<(), String> {
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
    fn execute_builtin(
        &self,
        state: &mut ShellState,
        invocation: CommandInvocation,
    ) -> Result<(), String> {
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

            "cd" => self.cd(state, invocation),

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
    fn cd(&self, state: &mut ShellState, invocation: CommandInvocation) -> Result<(), String> {
        let raw_target = if invocation.args.is_empty() {
            "~"
        } else {
            &invocation.args[0]
        };

        let target = self.expand_cd_target(state, raw_target)?;

        let old_dir = std::env::current_dir().map_err(|e| e.to_string())?;

        std::env::set_current_dir(&target).map_err(|e| format!("cd: {}: {}", target, e))?;

        let new_dir = std::env::current_dir().map_err(|e| e.to_string())?;

        state.previous_directory = Some(old_dir);

        state.cwd = new_dir.clone();

        if raw_target == "-" {
            println!("{}", new_dir.display());
        }

        Ok(())
    }
    fn expand_cd_target(&self, state: &ShellState, target: &str) -> Result<String, String> {
        match target {
            "-" => state
                .previous_directory
                .as_ref()
                .map(|p| p.to_string_lossy().to_string())
                .ok_or_else(|| "cd: OLDPWD not set".to_string()),

            "~" => std::env::var("HOME").map_err(|_| "HOME not set".to_string()),

            _ if target.starts_with("~/") => {
                let home = std::env::var("HOME").map_err(|_| "HOME not set".to_string())?;

                Ok(format!("{}/{}", home, &target[2..]))
            }

            _ => Ok(target.to_string()),
        }
    }
}
