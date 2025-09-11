use std::fmt::Formatter;
use std::process::{Command, Stdio};
use crate::display::display;
use crate::parse::parse;

macro_rules! run_error {
    ($($arg:tt)*) => {
        Err(RunError {error: format!($($arg)*)})
    };
}

pub struct RunError {
    pub error: String
}

impl std::fmt::Display for RunError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Run Error: {}", self.error)
    }
}

pub fn run() -> Result<(), RunError> {
    let cmd_result = Command::new("cargo")
        .arg("test")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    let cmd = match cmd_result {
        Ok(res) => res,
        Err(e) => {
            return run_error!("'cargo test' failed: {}", e.to_string())
        }
    };

    let output = match String::from_utf8(cmd.stdout) {
        Ok(res) => res,
        Err(_) => {
            return run_error!("failed to parse raw command output")
        }
    };
    
    match parse(output) {
        Ok(res) => display(res),
        Err(err) => return Err(err.to_run_error())
    }
    
    Ok(())
}