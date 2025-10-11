use crate::parse::{GeneralTestType, ParsedTestGroup, Status, Summary};

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
        let mut string: String = self.initial_message.clone() + "\n";

        let mut total_summary: Summary = Summary::default();

        for group in &self.test_groups {
            for test in group.tests.clone() {
                if test.status == Status::Ok {
                    string += format!(
                        "{} - {} {}\n",
                        "Pass".green(),
                        if test.test_type == GeneralTestType::Doc {
                            test.file_path.map_or("ERROR".to_string(), |x| x)
                        } else {
                            test.module_path
                        },
                        if test.test_type == GeneralTestType::Doc {
                            " Doc-test".blue()
                        } else {
                            "".to_string()
                        }
                    )
                    .as_str()
                } else if test.status == Status::Ignored {
                    string += format!(
                        "{} - {} {} {}\n",
                        "Ignored".yellow(),
                        if test.test_type == GeneralTestType::Doc {
                            test.file_path.map_or("ERROR".to_string(), |x| x)
                        } else {
                            test.module_path
                        },
                        test.ignore_reason
                            .clone()
                            .map_or("".to_string(), |x| format!("({})", x)),
                        if test.test_type == GeneralTestType::Doc {
                            " Doc-test".blue()
                        } else {
                            "".to_string()
                        }
                    )
                    .as_str()
                } else if test.status == Status::Failed {
                    string += format!(
                        "{} - {} - See reason below {}\n",
                        "Failed".red(),
                        if test.test_type == GeneralTestType::Doc {
                            test.file_path.map_or("ERROR".to_string(), |x| x)
                        } else {
                            test.module_path
                        },
                        if test.test_type == GeneralTestType::Doc {
                            " Doc-test".blue()
                        } else {
                            "".to_string()
                        }
                    )
                    .as_str()
                }
            }
        }

        string
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
