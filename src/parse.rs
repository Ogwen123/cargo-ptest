use crate::config::Config;
use crate::run::RunError;
use regex::Regex;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

macro_rules! parse_error {
    ($($args:tt)*) => {
        Err(ParseError { error: format!("ParseError: {}", format!($($args)*)) })
    };
}

macro_rules! map_parse_error {
    ($($args:tt)*) => {
        ParseError { error: format!("ParseError: {}", format!($($args)*)) }
    };
}

#[derive(PartialEq, Clone)]
enum Status {
    Ok,
    Failed,
    Ignored
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Status::Ok => "Ok".to_string(),
            Status::Failed => "Failed".to_string(),
            Status::Ignored => "Ignored".to_string()
        };

        write!(f, "{}", value)
    }
}

#[derive(Clone)]
struct RawTest {
    path: String,
    status: Status,
    note: Option<String>,
    error_reason: Option<String>
}

impl Display for RawTest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RawTest {{\n    path: {}\n    status: {}\n    note: {:?}\n    error_reason: {:?}\n  }}",
            self.path, self.status, self.note, self.error_reason
        )
    }
}

pub struct TestLeaf {
    pub name: String,
    pub success: bool
}

pub struct TestBranch {
    pub is_result: bool,
    pub name: String,
    pub children: Option<HashMap<String, TestBranch>>,
    pub result: Option<TestLeaf>
}

impl TestBranch {
    fn has(&self, name: String) {}
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

fn get_next<'a>(iter: &mut dyn Iterator<Item = &'a str>) -> Option<&'a str> {
    loop {
        let next = iter.next()?;

        if next.len() != 0 {
            return Some(next)
        }
    }
}

fn update_raw_test_error(raw_tests: &mut Vec<RawTest>, buffer: &String, name: &String) {
    println!("adding error to raw test");
    // add error message to raw test
    for i in raw_tests {
        if i.path == *name {
            i.error_reason = Some(buffer.clone());
            break
        }
    }
}

fn merge_outputs(stdout: String, stderr: String) -> HashMap<String, Vec<String>> {
    let block_beginning = Regex::new(r"running (\d+) test(s?)").unwrap();
    let running_message = Regex::new(r"Running (unittests )?(?<path>[\w\/.-]+) \((?<binpath>[\w\/.-]+)\)").unwrap();

    let windows_safe_out = stdout.replace("\r", ""); // remove any carriage returns windows might be adding
    let windows_safe_err = stderr.replace("\r", "");

    let mut err_lines = windows_safe_err.split("\n").filter(|x| {
        return if x.trim().starts_with("Running ") || x.trim().starts_with("Doc-tests") {
            true
        } else {
            false
        };
    });
    println!("huh");
    let mut lines = windows_safe_out.split("\n").filter(|x| x.len() != 0);

    println!("error lines: {:?}", err_lines.clone().collect::<Vec<&str>>());

    let mut blocks: HashMap<String, Vec<String>> = HashMap::new();
    let mut buffer: Vec<String> = Vec::new();

    lines.for_each(|x| {
        if block_beginning.is_match(&x) {
            let next = match get_next(&mut err_lines) {
                Some(res) => res,
                None => return
            };

            if buffer.len() > 0 {
                let capture = match running_message.captures(buffer[0].as_str()) {
                    Some(res) => res,
                    None => {
                        println!("{}", buffer[0]);
                        println!("could not find path in beginning message");
                        return
                    }
                };

                let path = &capture["path"];
                if path.len() == 0 {
                    println!("found nothing for path");
                    return
                }

                let _ = blocks.insert(path.to_string(), buffer[1..].to_vec());
            }

            buffer = Vec::new();
            buffer.push(next.trim().to_string());
            buffer.push(x.trim().to_string())
        } else {
            if buffer.len() > 0 { buffer.push(x.trim().to_string()) }
        }
    });
    blocks
}

// possible formats
// test path ... ok|FAILED|ignored
// test path - note ... ok|FAILED|ignored
// notes are things like "should panic"
fn build_raw_test(test_string: &str) -> Result<RawTest, String> {
    let split_test: Vec<&str> = test_string.split("...").collect::<Vec<&str>>();

    let path_info = split_test[0];

    let split_path: Vec<&str> = path_info
        .split(" ")
        .filter(|x| x.len() > 0)
        .collect::<Vec<&str>>();

    let path = split_path[1].to_string();

    let mut note: Option<String> = None;
    if path_info.contains("-") {
        let mut buffer = String::new();
        let mut found_dash: bool = false;
        for i in split_path {
            if found_dash {
                buffer += i;
                buffer += " ";
            } else {
                if i == "-" {
                    found_dash = true
                }
            }
        }
        note = Some(buffer.trim().to_string());
    }

    let status = match split_test[1].trim() {
        x if x.contains("ok") => Status::Ok,
        x if x.contains("FAILED") => Status::Failed,
        x if x.contains("ignored") => Status::Ignored,
        _ => Status::Ok
    };

    Ok(RawTest {
        path,
        status,
        note,
        error_reason: None
    })
}

fn build_tree(raw_tests: Vec<RawTest>) -> Vec<TestBranch> {
    let mut results: Vec<TestBranch>;

    // set base test branch

    for test in raw_tests {
        let path_elements = test
            .path
            .split("::")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let mut looking_at: Option<&mut TestBranch> = None;

        for (index, elem) in path_elements.iter().enumerate() {}
    }

    vec![TestBranch {
        is_result: false,
        name: String::new(),
        children: None,
        result: None
    }]
}

// possible endings - ok, FAILED, ignored

pub fn parse(
    stdout: String,
    stderr: String,
    config: &Config
) -> Result<Vec<TestBranch>, ParseError> {
    println!("{}", stdout);
    // regex
    let running_line_match =
        Regex::new(r"Running (unittests ?)(?<path>\w+) \((?<binpath>\w+)\)").unwrap();
    let count_line_match = Regex::new(r"running (?<count>\d+) test(s?)").unwrap();
    let test_run_match = Regex::new(r"running (?<count>.) tests").unwrap();
    let tests_summary_match = Regex::new(r"test result: (?<overall_result>.)\.. (?<passed>.)\. passed; (?<failed>.)\. failed; (?<ignored>.)\. ignored; (?<measured>.)\. measured; (?<filtered_out>.)\. filtered out; finished in (?<finish_time>.)\.s ").unwrap();

    let mut trees: Vec<TestBranch> = Vec::new();

    let windows_safe = stdout.replace("\r", ""); // remove any carriage returns windows might be adding
    let mut lines = windows_safe.split("\n").filter(|x| x.len() != 0);

    let mut test_blocks_found = 0;

    // loop {
    //     linesc += 1;
    //     let line = match get_next(&mut lines) {
    //         Some(res) => res,
    //         None => {
    //             println!("breaking after {} lines", linesc);
    //             break;
    //         }
    //     };
    //     println!("found line: {}", line);
    //     // skip lines until we get to the end of the build messages
    //     if line.trim().starts_with("Finished") {
    //         println!("found the finished line");
    //         break;
    //     }
    // }
    for (key, value) in merge_outputs(stdout, stderr) {
        println!("{}, {:?}", key, value);
    }
    loop {
        let mut failed = 0;
        let test_block_intro = match get_next(&mut lines) {
            Some(res) => res,
            None => break
        };

        println!("this is the res: {:?}", test_block_intro);

        let capture = match running_line_match.captures(test_block_intro) {
            Some(res) => res,
            None => {
                if test_blocks_found == 0 {
                    return parse_error!("Could not find any tests in the output.")
                } else {
                    break
                }
            }
        };

        test_blocks_found += 1;
        let count_str = &capture["count"];

        let count: usize = match count_str.parse::<usize>() {
            Ok(res) => res,
            Err(_) => {
                return parse_error!("Could not convert count to an number, got: {}", count_str)
            }
        };

        let mut raw_tests: Vec<RawTest> = Vec::new();

        for _ in 0..count {
            let test_string = get_next(&mut lines).unwrap();
            let raw_test = build_raw_test(test_string).map_err(|x| return map_parse_error!("{}", x))?;

            if raw_test.status == Status::Failed {
                failed += 1;
            }

            raw_tests.push(raw_test)
        }

        // add error reasons to raw types
        let mut in_failure_block = false;

        if failed > 0 {
            let mut add_to_buffer = false;
            let mut buffer = String::new();
            let mut name = String::new();
            loop {
                let line_option = get_next(&mut lines);

                let line = match line_option {
                    Some(res) => res,
                    None => break
                };

                if line.trim().starts_with("----") {
                    add_to_buffer = true;

                    if &buffer.len() != &0 {
                        update_raw_test_error(&mut raw_tests, &buffer, &name);
                    }

                    name = line
                        .replace("-", "")
                        .split(" ")
                        .filter(|x| x.len() != 0)
                        .collect::<Vec<&str>>()[0]
                        .to_string();

                    buffer = String::new();
                } else if line.trim().starts_with("failures:") {
                    if in_failure_block {
                        update_raw_test_error(&mut raw_tests, &buffer, &name);
                        break
                    } else {
                        in_failure_block = true;
                    }
                } else {
                    if add_to_buffer {
                        buffer += line;
                        buffer += "\n";
                    }
                }
            }

            // skip through the list of failed tests after the seconds failures:
            for _ in 0..failed {
                let _ = get_next(&mut lines);
            }
        }

        raw_tests.iter().for_each(|x| println!("{}", x));

        let test_block_summary = lines.next();
        println!("{:?}", test_block_summary);
        build_tree(raw_tests)
            .into_iter()
            .for_each(|x| trees.push(x));
    }
    println!("stopped finding");
    Ok(trees)
}
