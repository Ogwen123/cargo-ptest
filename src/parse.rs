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
    Ignored,
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Status::Ok => "Ok".to_string(),
            Status::Failed => "Failed".to_string(),
            Status::Ignored => "Ignored".to_string(),
        };

        write!(f, "{}", value)
    }
}

#[derive(Clone)]
enum TestType {
    Unit,
    Doc,
    Tests,
}

#[derive(Clone)]
struct Path {
    test_type: TestType,
    components: Vec<String>,
    crate_name: String,
    test_data: Vec<String>,
}

impl Path {
    fn new(stderr_line: String, test_data: Vec<String>) -> Result<Path, ParseError> {
        // the path is the crate name found in target/debug/deps/crate_name-xxxxxxxxxxxxxxxx
        // plus the file path found in Running unittests file/path.rs

        if stderr_line.contains("Doc-tests") {
            let crate_name = stderr_line.split(" ").collect::<Vec<&str>>()[1].to_string();
            Ok(Path {
                test_type: TestType::Doc,
                components: Vec::new(),
                crate_name,
                test_data,
            })
        } else {
            let stderr_message = Regex::new(r"Running (unittests )?(?<path>[\w\/.-]+) \(target\/debug\/deps\/(?<crate_name>[\w\/.-]+)-(?<hash>[\w]+)\)").unwrap();

            let capture = match stderr_message.captures(stderr_line.as_str()) {
                Some(res) => res,
                None => return parse_error!("Could not extract data from running line"),
            };

            let path = &capture["path"];
            let crate_name = &capture["crate_name"];

            if path.len() == 0 || crate_name.len() == 0 {
                return parse_error!("Could not extract information from running line.");
            }

            let mut test_type: TestType;

            if stderr_line.contains("unittest") {
                test_type = TestType::Unit
            } else {
                test_type = TestType::Tests
            }

            Ok(Path {
                test_type,
                components: path
                    .split("/")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
                crate_name: crate_name.to_string(),
                test_data,
            })
        }
    }

    fn joined_components(&self) -> String {
        self.components.join("/")
    }

    fn full_path(&self) -> String {
        self.crate_name.clone() + "/" + self.joined_components().as_str()
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.components.join("/"))
    }
}

#[derive(Clone)]
struct RawTest {
    path: String,
    status: Status,
    note: Option<String>,
    error_reason: Option<String>,
    ignore_reason: Option<String>,
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
    pub success: bool,
}

pub struct TestBranch {
    pub is_result: bool,
    pub name: String,
    pub children: Option<HashMap<String, TestBranch>>,
    pub result: Option<TestLeaf>,
}

impl TestBranch {
    fn has(&self, name: String) {}
}

#[derive(Debug)]
pub struct ParseError {
    pub error: String,
}

impl ParseError {
    pub fn to_run_error(&self) -> RunError {
        RunError {
            error: self.error.clone(),
        }
    }
}

fn get_next<'a>(iter: &mut dyn Iterator<Item = &'a str>) -> Option<&'a str> {
    loop {
        let next = iter.next()?;

        if next.len() != 0 {
            return Some(next);
        }
    }
}

fn update_raw_test_error(raw_tests: &mut Vec<RawTest>, buffer: &String, name: &String) {
    println!("adding error to raw test");
    // add error message to raw test
    for i in raw_tests {
        if i.path == *name {
            i.error_reason = Some(buffer.clone());
            break;
        }
    }
}

fn merge_outputs(stdout: String, stderr: String) -> Result<Vec<Path>, ParseError> {
    let block_beginning = Regex::new(r"running (\d+) test(s?)").unwrap();

    let windows_safe_out = stdout.replace("\r", ""); // remove any carriage returns windows might be adding
    let windows_safe_err = stderr.replace("\r", "");

    let mut err_lines = windows_safe_err.split("\n").filter(|x| {
        return if x.trim().starts_with("Running ") || x.trim().starts_with("Doc-tests") {
            true
        } else {
            false
        };
    });

    let mut lines = windows_safe_out.split("\n").filter(|x| x.len() != 0);

    let mut blocks: Vec<Path> = Vec::new();
    let mut buffer: Vec<String> = Vec::new();

    for x in lines {
        if block_beginning.is_match(&x) {
            let next = match get_next(&mut err_lines) {
                Some(res) => res,
                None => continue,
            };

            // when the start of a block is found push the buffer with the previous block to blocks and start the new block
            if buffer.len() > 0 {
                blocks.push(Path::new(buffer[0].clone(), buffer[1..].to_vec()).map_err(|x| x)?)
            }

            buffer = Vec::new();
            buffer.push(next.trim().to_string());
            buffer.push(x.trim().to_string());
        } else {
            if buffer.len() > 0 {
                buffer.push(x.trim().to_string());
            }
        }
    }
    Ok(blocks)
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

    let status_string = split_test[1].trim();

    let status = match &status_string {
        x if x.contains("ok") => Status::Ok,
        x if x.contains("FAILED") => Status::Failed,
        x if x.contains("ignored") => Status::Ignored,
        _ => Status::Ok,
    };

    let mut ignore_reason: Option<String> = None;

    if status == Status::Ignored && status_string.contains(", ") {
        ignore_reason = Some(status_string.split(", ").collect::<Vec<&str>>()[1].to_string())
    }

    Ok(RawTest {
        path,
        status,
        note,
        error_reason: None,
        ignore_reason,
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
        result: None,
    }]
}

// possible endings - ok, FAILED, ignored

pub fn parse(
    stdout: String,
    stderr: String,
    config: &Config,
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
    for path in merge_outputs(stdout, stderr).unwrap() {
        println!("{}\n{:?}\n\n\n", path.full_path(), path.test_data);
    }
    loop {
        let mut failed = 0;
        let test_block_intro = match get_next(&mut lines) {
            Some(res) => res,
            None => break,
        };

        println!("this is the res: {:?}", test_block_intro);

        let capture = match running_line_match.captures(test_block_intro) {
            Some(res) => res,
            None => {
                if test_blocks_found == 0 {
                    return parse_error!("Could not find any tests in the output.");
                } else {
                    break;
                }
            }
        };

        test_blocks_found += 1;
        let count_str = &capture["count"];

        let count: usize = match count_str.parse::<usize>() {
            Ok(res) => res,
            Err(_) => {
                return parse_error!("Could not convert count to an number, got: {}", count_str);
            }
        };

        let mut raw_tests: Vec<RawTest> = Vec::new();

        for _ in 0..count {
            let test_string = get_next(&mut lines).unwrap();
            let raw_test =
                build_raw_test(test_string).map_err(|x| return map_parse_error!("{}", x))?;

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
                    None => break,
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
                        break;
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

            // skip through the list of failed tests after the second 'failures':
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
