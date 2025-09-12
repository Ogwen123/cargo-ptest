use std::collections::HashMap;
use regex::Regex;
use crate::run::RunError;

macro_rules! parse_error {
    ($($args:tt)*) => {
        Err(ParseError { error: format!("ParseError: {}", format!($($args)*)) })
    };
}

pub struct TestResult {
    pub name: String,
    pub success: bool
}

pub struct ResultOption {
    pub children: HashMap<String, ResultOption>,
    pub result: TestResult
}

pub struct ParseError {
    pub error: String
}

impl ParseError {
    pub fn to_run_error(&self) -> RunError {
        RunError {
            error: self.error.clone()
        }
    }
}

// possible endings - ok, FAILED, ignored

// keep looping until we stop finding running x test messages

pub fn parse(output: String) -> Result<HashMap<String, ResultOption>, ParseError> {
    println!("{}", output);
    // regexes
    let count_line_match = Regex::new(r"running (?<count>[0-9]+) tests").unwrap();
    let test_run_match = Regex::new(r"running (?<count>.) tests").unwrap();
    let tests_summary_match = Regex::new(r"test result: (?<overall_result>.)\.. (?<passed>.)\. passed; (?<failed>.)\. failed; (?<ignored>.)\. ignored; (?<measured>.)\. measured; (?<filtered_out>.)\. filtered out; finished in (?<finish_time>.)\.s ").unwrap();

    let mut tree: HashMap<String, ResultOption> = HashMap::new();

    let windows_safe = output.replace("\r", ""); // remove any carriage returns windows might be adding

    let mut lines  = windows_safe.split("\n").filter(|x| x.len() != 0);

    let test_blocks_found = 0;

    loop {
        let test_block_intro = lines.next().unwrap().trim();
        let capture = match count_line_match.captures(test_block_intro) {
            Some(res) => res,
            None => {
                if test_blocks_found == 0 {
                    return parse_error!("could not find any tests in the input")
                } else {
                    break
                }
            }
        };

        let count_str = &capture["count"];

        let count: usize = match count_str.parse::<usize>() {
            Ok(res) => res,
            Err(_) => return parse_error!("Could not convert count to an number, got: {}", count_str)
        };
        
    }

    Ok(tree)
}