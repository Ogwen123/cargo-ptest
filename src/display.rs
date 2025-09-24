use crate::config::Config;
use crate::parse::TestBranch;
use std::collections::HashMap;

pub enum Colour {
    GREEN,
    RED,
    ORANGE,
}

pub struct Test {
    name: String,
    result: String,
}

pub fn colour<T>(colour: Colour, text: T) -> String
where
    T: ToString,
{
    return String::from("test");
}

pub struct DisplayTree {
    pub initial_message: String,
    pub branches: HashMap<String, String>,
}

pub fn display(tree: Vec<TestBranch>, config: &Config) {}
