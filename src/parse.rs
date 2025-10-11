use crate::run::RunError;
use regex::Regex;
use std::fmt::{Debug, Display, Formatter};

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
pub enum Status {
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

#[derive(Clone, PartialEq)]
pub enum TestType {
    Unit,
    Doc,
    Tests,
}

impl Display for TestType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TestType::Unit => "Unit",
                TestType::Doc => "Doc",
                TestType::Tests => "Tests",
            }
        )
    }
}

#[derive(Clone, PartialEq)]
pub enum GeneralTestType {
    Normal,
    Doc,
}

impl Display for GeneralTestType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GeneralTestType::Normal => "Normal",
                GeneralTestType::Doc => "Doc",
            }
        )
    }
}

#[derive(Clone)]
pub struct RawTestGroup {
    test_type: TestType,
    file_path: Vec<String>,
    crate_name: String,
    test_data: Vec<String>,
}

impl RawTestGroup {
    fn new(
        stderr_line: String,
        test_data: Vec<String>,
        is_doc_test: bool,
    ) -> Result<RawTestGroup, ParseError> {
        // the path is the crate name found in target/debug/deps/crate_name-xxxxxxxxxxxxxxxx
        // plus the file path found in Running unittests file/path.rs
        // if the test type is Testing then the crate name is the previous unittests crate name

        if is_doc_test {
            Ok(RawTestGroup {
                test_type: TestType::Doc,
                file_path: Vec::new(),
                crate_name: "Doc-tests".to_string(),
                test_data,
            })
        } else {
            let stderr_message = Regex::new(r"Running (unittests )?(?<path>[\w/.-]+) \(target/debug/deps/(?<crate_name>[\w/.-]+)-(?<hash>[\w]+)\)").unwrap();

            let capture = match stderr_message.captures(stderr_line.as_str()) {
                Some(res) => res,
                None => {
                    return parse_error!(
                        "Could not extract data from running line, got \"{}\"",
                        stderr_line
                    );
                }
            };

            let path = &capture["path"];
            let crate_name = &capture["crate_name"];

            if path.len() == 0 || crate_name.len() == 0 {
                return parse_error!("No data found in running line.");
            }

            let mut test_type: TestType;

            if stderr_line.contains("unittest") {
                test_type = TestType::Unit
            } else {
                test_type = TestType::Tests
            }

            Ok(RawTestGroup {
                test_type,
                file_path: path
                    .split("/")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
                crate_name: crate_name.to_string(),
                test_data,
            })
        }
    }

    fn joined_components(&self) -> String {
        self.file_path.join("/")
    }

    fn full_path(&self) -> String {
        self.crate_name.clone() + "/" + self.joined_components().as_str()
    }
}

impl Display for RawTestGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} is type {}", self.file_path.join("/"), self.test_type)
    }
}

#[derive(Clone)]
pub struct ParsedTest {
    pub test_type: GeneralTestType,
    pub module_path: String,
    pub status: Status,
    pub file_path: Option<String>,
    pub note: Option<String>,
    pub error_reason: Option<String>,
    pub ignore_reason: Option<String>,
}

impl ParsedTest {
    fn new(test_line: String) -> Result<ParsedTest, ParseError> {
        // write regex to validate test line, path, status, note, etc
        let test_line_match = Regex::new(
            r"test (?<module_path>[\w:_]+)( - (?<note>[\w\s]+))? ... (?<status>FAILED|ignored|ok)(, (?<ignore_reason>[\w\s]+))?",
        ).unwrap();
        let doc_test_line = Regex::new(r"test (?<file_path>[\w/.]+) -( (?<module_path>[\w/:]+))? \(line (?<line_num>[\d]+)\)( - (?<note>[\w\s]+))? \.\.\. (?<status>[\w]+)").unwrap();

        if test_line_match.is_match(test_line.as_str()) {
            let capture = match test_line_match.captures(test_line.as_str()) {
                Some(res) => res,
                None => {
                    return parse_error!(
                        "Data could not be extracted from provided test line, got \"{}\"",
                        test_line
                    );
                }
            };

            let path = match capture.name("module_path") {
                Some(res) => res.as_str().to_string(),
                None => {
                    return parse_error!(
                        "Could not extract module path from test line, test line is {}",
                        test_line
                    );
                }
            };
            let status_string = match capture.name("status") {
                Some(res) => res.as_str(),
                None => {
                    return parse_error!(
                        "Could not extract status from test line, test line is {}",
                        test_line
                    );
                }
            };

            let status = match &status_string {
                x if x.contains("ok") => Status::Ok,
                x if x.contains("FAILED") => Status::Failed,
                x if x.contains("ignored") => Status::Ignored,
                _ => Status::Ok,
            };

            Ok(ParsedTest {
                test_type: GeneralTestType::Normal,
                module_path: path,
                status,
                file_path: None,
                note: capture
                    .name("note")
                    .map_or(None, |x| Some(x.as_str().to_string())),
                error_reason: None,
                ignore_reason: capture
                    .name("ignore_reason")
                    .map_or(None, |x| Some(x.as_str().to_string())),
            })
        } else if doc_test_line.is_match(test_line.as_str()) {
            let capture = match doc_test_line.captures(test_line.as_str()) {
                Some(res) => res,
                None => {
                    return parse_error!(
                        "Data could not be extracted from provided Doc-test line, got \"{}\"",
                        test_line
                    );
                }
            };

            let module_path = match capture.name("module_path") {
                Some(res) => res.as_str().to_string(),
                None => String::new(),
            };

            let file_path = match capture.name("file_path") {
                Some(res) => res.as_str().to_string(),
                None => {
                    return parse_error!(
                        "Could not extract file path from Doc-test line, test line is \"{}\"",
                        test_line
                    );
                }
            };

            let status_string = match capture.name("status") {
                Some(res) => res.as_str(),
                None => {
                    return parse_error!(
                        "Could not extract status from test line, test line is \"{}\"",
                        test_line
                    );
                }
            };

            let status = match &status_string {
                x if x.contains("ok") => Status::Ok,
                x if x.contains("FAILED") => Status::Failed,
                x if x.contains("ignored") => Status::Ignored,
                _ => Status::Ok,
            };

            Ok(ParsedTest {
                test_type: GeneralTestType::Doc,
                module_path,
                status,
                file_path: Some(file_path),
                note: capture
                    .name("note")
                    .map_or(None, |x| Some(x.as_str().to_string())),
                error_reason: None,
                ignore_reason: None,
            })
        } else {
            parse_error!(
                "Provided string wasn't normal test line or a Doc-test line, got {}",
                test_line
            )
        }
    }

    fn add_error_reason(&mut self, error_reason: String) {
        self.error_reason = Some(error_reason)
    }
}

impl Display for ParsedTest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nParsedTest {{\n    test_type: {}\n    module_path: {}\n    status: {}\n    file_path: {:?}\n    note: {:?}\n    error_reason: {:?}\n    ignore_reason: {:?}\n}}",
            self.test_type,
            self.module_path,
            self.status,
            self.file_path,
            self.note,
            self.error_reason,
            self.ignore_reason
        )
    }
}

impl Debug for ParsedTest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub struct Summary {
    pub status: Status,
    pub passed: u32,
    pub failed: u32,
    pub ignored: u32,
    pub measured: u32,
    pub filtered: u32,
    pub time: f64,
}

impl Summary {
    fn new(summary_line: &str) -> Result<Summary, ParseError> {
        let tests_summary_line = Regex::new(r"test result: (?<overall_result>\w+)\. (?<passed>\d+) passed; (?<failed>\d+) failed; (?<ignored>\d+) ignored; (?<measured>\d+) measured; (?<filtered_out>\d+) filtered out; finished in (?<finish_time>[\d.]+)s").unwrap();

        if !tests_summary_line.is_match(summary_line) {
            return parse_error!(
                "Data could not be extracted from provided summary line, got \"{}\"",
                summary_line
            );
        }

        let capture = match tests_summary_line.captures(summary_line) {
            Some(res) => res,
            None => {
                return parse_error!(
                    "Data could not be extracted from provided summary line, got \"{}\"",
                    summary_line
                );
            }
        };

        // if a value cannot be parsed from the regex then just use 0 rather than erroring
        Ok(Summary {
            status: match &capture["overall_result"] {
                "ok" => Status::Ok,
                "FAILED" => Status::Failed,
                _ => {
                    return parse_error!(
                        "Status extracted from summary line could not be recognised, got {}",
                        &capture["overall_result"]
                    );
                }
            },
            passed: match &capture["passed"].parse::<u32>() {
                Ok(res) => res.clone(),
                Err(_) => 0,
            },
            failed: match &capture["failed"].parse::<u32>() {
                Ok(res) => res.clone(),
                Err(_) => 0,
            },
            ignored: match &capture["ignored"].parse::<u32>() {
                Ok(res) => res.clone(),
                Err(_) => 0,
            },
            measured: match &capture["measured"].parse::<u32>() {
                Ok(res) => res.clone(),
                Err(_) => 0,
            },
            filtered: match &capture["filtered_out"].parse::<u32>() {
                Ok(res) => res.clone(),
                Err(_) => 0,
            },
            time: match &capture["finish_time"].parse::<f64>() {
                Ok(res) => res.clone(),
                Err(_) => 0.0,
            },
        })
    }
}

impl Default for Summary {
    fn default() -> Self {
        Summary {
            status: Status::Failed,
            passed: 0,
            failed: 0,
            ignored: 0,
            measured: 0,
            filtered: 0,
            time: 0.0,
        }
    }
}

pub struct ParsedTestGroup {
    pub crate_name: String,
    pub file_path: Vec<String>,
    pub tests: Vec<ParsedTest>,
    pub summary: Option<Summary>,
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

fn merge_outputs(stdout: String, stderr: String) -> Result<Vec<RawTestGroup>, ParseError> {
    let block_beginning = Regex::new(r"running (\d+) test(s?)").unwrap();
    let doc_test_beginning = Regex::new(r"Doc-tests (?<crate>[\w-]+)").unwrap();

    let windows_safe_out = stdout.replace("\r", ""); // remove any carriage returns windows might be adding
    let windows_safe_err = stderr.replace("\r", "");

    let mut err_lines = windows_safe_err.split("\n").filter(|x| {
        return if x.trim().starts_with("Running ") || x.trim().starts_with("Doc-tests") {
            true
        } else {
            false
        };
    });

    let lines = windows_safe_out.split("\n").filter(|x| x.len() != 0);

    let mut blocks: Vec<RawTestGroup> = Vec::new();
    let mut buffer: Vec<String> = Vec::new();
    let mut reached_doc_tests = false;

    for x in lines {
        if reached_doc_tests {
            buffer.push(x.trim().to_string());
            continue;
        }

        if block_beginning.is_match(&x) {
            let next = match get_next(&mut err_lines) {
                Some(res) => res,
                None => continue,
            };

            // when the start of a block is found push the buffer with the previous block to blocks and start the new block
            if buffer.len() > 0 {
                blocks.push(
                    RawTestGroup::new(buffer[0].clone(), buffer[1..].to_vec(), false)
                        .map_err(|x| x)?,
                )
            }

            buffer = Vec::new();

            if doc_test_beginning.is_match(next) {
                reached_doc_tests = true;
                buffer.push(x.trim().to_string());
                continue;
            }

            buffer.push(next.trim().to_string());
            buffer.push(x.trim().to_string());
        } else {
            if buffer.len() > 0 {
                buffer.push(x.trim().to_string());
            }
        }
    }
    if reached_doc_tests {
        blocks.push(RawTestGroup::new(String::new(), buffer, reached_doc_tests).map_err(|x| x)?);
    } else {
        blocks.push(RawTestGroup::new(buffer[0].clone(), buffer[1..].to_vec(), reached_doc_tests).map_err(|x| x)?);
    }
    
    //DEBUG
    println!("blocks: {}", blocks.len());
    Ok(blocks)
}

// possible endings - ok, FAILED, ignored

pub fn parse(stdout: String, stderr: String) -> Result<Vec<ParsedTestGroup>, ParseError> {
    println!("{}", stdout);
    // regex
    let test_block_start_match = Regex::new(r"running (?<count>\d+) test(s?)").unwrap();
    let doc_test_line = Regex::new(r"test (?<file_path>[\w/.]+) -( (?<module_path>[\w/:]+))? \(line (?<line_num>\d+)\)( - (?<note>[\w\s]+))? \.\.\. (?<status>\w+)").unwrap();

    for path in merge_outputs(stdout.clone(), stderr.clone()).map_err(|x| x)? {
        println!("{}\n{:?}\n\n\n", path, path.test_data);
    }

    let mut parsed_groups: Vec<ParsedTestGroup> = Vec::new();
    let groups = merge_outputs(stdout, stderr).map_err(|x| x)?;

    for group in groups {
        let mut failed = 0;

        let mut line_iter = group.test_data.iter().map(|x| x.as_str());
        let mut parsed_tests: Vec<ParsedTest> = Vec::new();
        if group.test_type == TestType::Doc {
            // when parsing doc tests just look for lines starting with test and parse them
            loop {
                let line = match get_next(&mut line_iter) {
                    Some(res) => res,
                    None => break,
                };

                if doc_test_line.is_match(line) {
                    parsed_tests.push(ParsedTest::new(line.to_string()).map_err(|x| x)?)
                }
            }
            parsed_groups.push(ParsedTestGroup {
                crate_name: group.crate_name.clone(),
                file_path: group.file_path.clone(),
                tests: parsed_tests,
                summary: None,
            })
        } else {
            //DEBUG
            println!("doing: {}", group);
            let test_block_start = match get_next(&mut line_iter) {
                Some(res) => res,
                None => break,
            };

            //DEBUG
            println!("found line {}", test_block_start);

            let capture = match test_block_start_match.captures(test_block_start) {
                Some(res) => res,
                None => {
                    //DEBUG
                    println!("huh");
                    break;
                }
            };
            let count_str = &capture["count"];

            let count: usize = match count_str.parse::<usize>() {
                Ok(res) => res,
                Err(_) => {
                    return parse_error!(
                        "Could not convert the captured number from a test block, got: {}",
                        count_str
                    );
                }
            };
            //DEBUG
            println!("found count {}", count);
            // parse each test line
            for _ in 0..count {
                let line = match get_next(&mut line_iter) {
                    Some(res) => res,
                    None => break,
                };
                let parsed_test = ParsedTest::new(line.to_string()).map_err(|x| x)?;
                if parsed_test.status == Status::Failed {
                    failed += 1
                }
                parsed_tests.push(parsed_test);
            }

            // parse the error reasons under "failures:" and add them to the correct tests
            let mut in_failure_block = false;

            if failed > 0 {
                let mut add_to_buffer = false;
                let mut buffer = String::new();
                let mut name = String::new();
                let failure_title =
                    Regex::new(r"---- (?<path>[\w:_]+) (?<channel>\w+) ----").unwrap();
                loop {
                    let line_option = get_next(&mut line_iter);
                    let line = match line_option {
                        Some(res) => res,
                        None => break,
                    }
                    .trim();
                    //DEBUG
                    println!("fail line: {}", line);

                    if failure_title.is_match(line) {
                        add_to_buffer = true;

                        if buffer.len() != 0 {
                            for i in &mut parsed_tests {
                                if i.module_path == name {
                                    i.add_error_reason(buffer.clone())
                                }
                            }
                        }

                        name = match failure_title.captures(line) {
                            Some(res) => res,
                            None => break,
                        }["path"]
                            .to_string();

                        buffer = String::new();
                    } else if line.starts_with("failures:") {
                        // catches the second "failures:"
                        if in_failure_block {
                            for i in &mut parsed_tests {
                                if i.module_path == name {
                                    i.add_error_reason(buffer.clone())
                                }
                            }
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

                // skip through the list of failed tests after the second "failures:"
                for _ in 0..failed {
                    let _temp = get_next(&mut line_iter);
                    //DEBUG
                    println!("temp: {:?}", _temp);
                }
            }
            //DEBUG
            println!("tests: {:?}", parsed_tests);
            let summary = match get_next(&mut line_iter) {
                Some(res) => res,
                None => {
                    return parse_error!(
                        "Could not extract summary data for {}",
                        group.full_path()
                    );
                }
            };
            println!("summary: {:?}, {}", summary, group.test_type);
            parsed_groups.push(ParsedTestGroup {
                crate_name: group.crate_name.clone(),
                file_path: group.file_path.clone(),
                tests: parsed_tests,
                summary: Some(Summary::new(summary).map_err(|x| x)?),
            })
        }
    }
    println!("stopped finding");
    //DEBUG
    let mut s = 0;

    parsed_groups.iter().for_each(|x| s += x.tests.len());
    println!("tests parsed: {}", s);
    Ok(parsed_groups)
}
