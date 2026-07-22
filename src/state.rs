use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Child;

// #[derive(Debug)]
// pub enum JobStatus {
//     Running,
//     Stopped,
//     Done,
// }

// #[derive(Debug)]
// pub struct Job {
//     pub id: usize,
//     pub pid: u32,
//     pub command: String,
//     pub status: JobStatus,
// }

// #[derive(Debug)]
// pub struct BackgroundProcess {
//     pub pid: u32,
//     pub command: String,
//     pub child: Child,
// }

#[derive(Debug)]
pub struct ShellState {
    pub cwd: PathBuf,
    pub previous_directory: Option<PathBuf>,
    // pub variables: HashMap<String, String>,
    // pub exported: HashMap<String, String>,
    //pub jobs: Vec<Job>,
    // pub background_processes: Vec<BackgroundProcess>,
}

impl ShellState {
    pub fn new() -> Self {
        Self {
            cwd: std::env::current_dir().unwrap(),
            previous_directory: None,
            // variables: HashMap::new(),
            // exported: HashMap::new(),
            //jobs: Vec::new(),
            // background_processes: Vec::new(),
        }
    }
}