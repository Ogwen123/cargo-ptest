use std::fmt::Formatter;
use std::process::{Command, Stdio};

macro_rules! run_error {
    ($($arg:tt)*) => {
        Err(RunError {error: format!($($arg)*)})
    };
}

pub struct RunError {
    error: String
}

impl std::fmt::Display for RunError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Run Error: {}", self.error)
    }
}

pub fn run() -> Result<String, RunError> {
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

    Ok(output)
}