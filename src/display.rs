use crate::parse::{GeneralTestType, ParsedTestGroup, Status, Summary};
use std::ops::Add;

trait Colourise {
    fn green(&self) -> String;
    fn red(&self) -> String;
    fn yellow(&self) -> String;
    fn blue(&self) -> String;
}

impl Colourise for &str {
    fn green(&self) -> String {
        String::from("\x1b[32m") + &*self.to_string() + "\x1b[0m"
    }

    fn red(&self) -> String {
        String::from("\x1b[31m") + &*self.to_string() + "\x1b[0m"
    }

    fn yellow(&self) -> String {
        String::from("\x1b[33m") + &*self.to_string() + "\x1b[0m"
    }

    fn blue(&self) -> String {
        String::from("\x1b[34m") + &*self.to_string() + "\x1b[0m"
    }
}

pub enum Colour {
    GREEN,
    RED,
    ORANGE,
}

pub enum DisplayType {
    Tree,
    Linear,
    Json,
}

pub enum Pipes {
    T,
    L,
    Vertical,
    Horizontal,
}

impl Pipes {
    fn d(&self) -> String {
        // keep the method name short to not clutter the display functions
        match self {
            Pipes::T => String::from("├"),
            Pipes::L => String::from("└"),
            Pipes::Vertical => String::from("│"),
            Pipes::Horizontal => String::from("─"),
        }
    }
}

pub struct StringBuilder {
    lines: Vec<String>,
    // a string to be added to the start of each line that is added through add()
    prefix: String,
    // a string to be added to the end of each line that is added through add(), e.g. \n
    suffix: String,
}

impl StringBuilder {
    fn new<A, B, C>(initial_message: A, prefix: B, suffix: C) -> Self
    where
        A: ToString,
        B: ToString,
        C: ToString,
    {
        StringBuilder {
            lines: vec![initial_message.to_string()],
            prefix: prefix.to_string(),
            suffix: suffix.to_string(),
        }
    }

    fn add<T>(&mut self, line: T)
    where
        T: ToString + AsRef<str>,
    {
        self.lines.push(
            self.prefix
                .clone()
                .add(line.as_ref())
                .add(self.suffix.as_str()),
        )
    }

    fn string(&self) -> String {
        self.lines.join("")
    }
}

pub struct Display {
    initial_message: String,
    test_groups: Vec<ParsedTestGroup>,
}

impl Display {
    pub fn new(initial_message: &str, parsed: Vec<ParsedTestGroup>) -> Display {
        Display {
            initial_message: initial_message.to_string(),
            test_groups: parsed,
        }
    }
    pub fn colour(c: Colour, s: &str) -> String {
        String::from("test")
    }
    fn tree(&self) -> String {
        String::new()
    }
    fn linear(&self) -> String {
        let mut sb: StringBuilder = StringBuilder::new(
            self.initial_message.clone() + "\n",
            Pipes::T.d() + " ",
            "\n",
        );

        let mut total_summary: Summary = Summary::default();

        for group in &self.test_groups {
            for test in group.tests.clone() {
                if test.test_type == GeneralTestType::Normal {
                    if test.status == Status::Ok {
                        sb.add(format!("{} - {}", "Pass".green(), test.module_path))
                    } else if test.status == Status::Ignored {
                        sb.add(format!(
                            "{} - {} {}",
                            "Ignored".yellow(),
                            test.module_path,
                            test.ignore_reason
                                .clone()
                                .map_or("".to_string(), |x| format!("({})", x)),
                        ))
                    } else if test.status == Status::Failed {
                        sb.add(format!(
                            "{} - {} - See reason below",
                            "Failed".red(),
                            test.module_path
                        ))
                    }
                } else {
                    if test.status == Status::Ok {
                        sb.add(format!(
                            "{} - {} from {} {}",
                            "Pass".green(),
                            test.module_path,
                            test.file_path.map_or("ERROR".to_string(), |x| x),
                            " Doc-test".blue()
                        ))
                    } else if test.status == Status::Ignored {
                        sb.add(format!(
                            "{} - {} from {} {} {}",
                            "Ignored".yellow(),
                            test.module_path,
                            test.file_path.map_or("ERROR".to_string(), |x| x),
                            test.ignore_reason
                                .clone()
                                .map_or("".to_string(), |x| format!("({})", x)),
                            " Doc-test".blue()
                        ))
                    } else if test.status == Status::Failed {
                        sb.add(format!(
                            "{} - {} from {} - See reason below {}",
                            "Failed".red(),
                            test.module_path,
                            test.file_path.map_or("ERROR".to_string(), |x| x),
                            " Doc-test".blue()
                        ))
                    }
                }
            }
        }

        sb.string()
    }
    fn json(&self) -> String {
        String::new()
    }
    pub fn display(&self, _type: DisplayType) {
        match _type {
            DisplayType::Tree => println!("{}", self.tree()),
            DisplayType::Linear => println!("{}", self.linear()),
            DisplayType::Json => println!("{}", self.json()),
        }
    }
}
