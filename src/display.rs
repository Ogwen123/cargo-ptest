use crate::parse::ParsedTestGroup;

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

pub struct Display {
    test_groups: Vec<ParsedTestGroup>
}

impl Display {
    pub fn new(parsed: Vec<ParsedTestGroup>) -> Display {
        Display {
            test_groups: Vec::new()
        }
    }
    pub fn colour(c: Colour, s: &str) -> String {
        String::new()
    }
    fn tree(&self) -> String {
        String::new()
    }
    fn linear(&self) -> String {
        String::new()
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
