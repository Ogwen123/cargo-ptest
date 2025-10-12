use crate::config::config;
use crate::parse::{parse, ParsedTestGroup};
use std::fmt::Formatter;
use std::process::{Command, Stdio};

macro_rules! run_error {
    ($($arg:tt)*) => {
        Err(RunError {error: format!("Run Error: {}", format!($($arg)*))})
    };
}

pub struct RunError {
    pub error: String,
}

impl std::fmt::Display for RunError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[31m{}\x1b[0m", self.error)
    }
}

fn help() {
    println!("ptest help");
    println!("run using \x1b[34mcargo ptest\x1b[0m");
    println!("all arguments passed to cargo ptest are forwarded to cargo test");
    println!("below is the help output for cargo test");
    println!();
}

pub fn run() -> Result<Vec<ParsedTestGroup>, RunError> {
    let unfiltered_args: Vec<String> = std::env::args().collect();

    let args: Vec<String>;

    // when running cargo ptest the args look like ["C:\\Users\\user\\.cargo\\bin\\cargo-ptest.exe", "ptest", ...]
    // when running cargo-ptest the args look like ["cargo-ptest"]

    if unfiltered_args[0] == "cargo-ptest" {
        args = unfiltered_args[1..].to_vec();
    } else if unfiltered_args[1] == "ptest" {
        args = unfiltered_args[2..].to_vec();
    } else {
        return run_error!("how did you manage to see this error");
    }

    //let mut complete_args: Vec<String> = vec!["--tests".to_string(), "--no-fail-fast".to_string()]; // --no-fail-fast makes sure all the unit, integration and docs tests are run
    let mut forward_args: Vec<String> = Vec::new();
    let mut consume_args: Vec<String> = Vec::new();

    let mut passed_forward_point: bool = false;

    // filter out the --no-capture args as it makes the output of the cargo test command unpredictable and messes with the parser
    // verbose also messes up parsing so it gets filtered out
    // remove any color so I can set color=never to avoid having to deal with ansi codes all over the place
    let filter_list = [
        "--nocapture",
        "-v",
        "--verbose",
        "--color=always",
        "--color=auto",
        "--color=never",
    ];

    args.into_iter().for_each(|x| {
        if passed_forward_point {
            if !filter_list.contains(&x.as_str()) {
                forward_args.push(x)
            }
        } else {
            if x == "--" {
                passed_forward_point = true;
            } else {
                consume_args.push(x);
            }
        }
    });

    forward_args.push("--color=never".to_string());

    //println!("{:?}", forward_args);
    //println!("{:?}", consume_args);

    let config = match config(consume_args) {
        Ok(res) => res,
        Err(err) => return Err(RunError { error: err }),
    };

    let cmd_result = Command::new("cargo")
        .arg("test")
        .args(&forward_args)
        .env("CARGO_TERM_COLOR", "always")
        .env("FORCE_COLOR", "1")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output();

    let cmd = match cmd_result {
        Ok(res) => res,
        Err(e) => return run_error!("'cargo test' failed: {}", e.to_string()),
    };

    let stdout = match String::from_utf8(cmd.stdout) {
        Ok(res) => res,
        Err(_) => return run_error!("failed to parse stdout from utf8"),
    };

    let stderr = match String::from_utf8(cmd.stderr) {
        Ok(res) => res,
        Err(_) => return run_error!("failed to parse stderr from utf8"),
    };

    if forward_args.contains(&"--help".to_string()) || forward_args.contains(&"-h".to_string()) {
        help();
        println!("{}", stdout);
        return Ok(Vec::new());
    }

    let parsed = match parse(stdout, stderr) {
        Ok(res) => res,
        Err(err) => return Err(err.to_run_error()),
    };

    Ok(parsed)
}
