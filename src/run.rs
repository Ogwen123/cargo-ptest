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

fn help() {
    println!("ptest help");
    println!("run using \x1b[34mcargo ptest\x1b[0m");
    println!("all arguments passed to cargo ptest are forwarded to cargo test");
    println!("below is the help output for cargo test");
    println!();
}

pub fn run() -> Result<(), RunError> {

    let unfiltered_args: Vec<String> = std::env::args().collect();

    let mut args: Vec<String>;

    // when running cargo ptest the args look like ["C:\\Users\\user\\.cargo\\bin\\cargo-ptest.exe", "ptest", ...]
    // when running cargo-ptest the args look like ["cargo-ptest"]

    if unfiltered_args[0] == "cargo-ptest" {
        args = unfiltered_args[1..].to_vec();
    } else if unfiltered_args[1] == "ptest" {
        args = unfiltered_args[2..].to_vec();
    } else {
        return run_error!("how did you manage to see this error")
    }

    //let mut complete_args: Vec<String> = vec!["--tests".to_string(), "--no-fail-fast".to_string()]; // --no-fail-fast makes sure all the unit, integration and docs tests are run
    let mut complete_args: Vec<String> = Vec::new();

    complete_args.append(&mut args);

    let has_double_dash = complete_args.iter().any(|x| x == "--");

    // filter out the --no-capture args as it makes the output of the cargo test command unpredictable and messes with the parser
    complete_args = complete_args.iter().filter_map(|x| if x != &"--nocapture" { Some(x.to_string()) } else { None } ).collect();
    
    if !has_double_dash {
        complete_args.push("--".to_string())
    }

    complete_args.push("--color=always".to_string());
    
    println!("{:?}", complete_args);
    let cmd_result = Command::new("cargo")
        .arg("test")
        .args(&complete_args)
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

    if complete_args.contains(&"--help".to_string()) || complete_args.contains(&"-h".to_string()) {
        help()
    }
    
    match parse(output) {
        Ok(res) => display(res),
        Err(err) => return Err(err.to_run_error())
    }
    
    Ok(())
}