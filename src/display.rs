use crate::config::Config;
use std::collections::HashMap;
use crate::parse::ParsedTestGroup;

pub enum Colour {
    GREEN,
    RED,
    ORANGE,
}

pub struct Display {}

impl Display {
    pub fn new(parsed: Vec<ParsedTestGroup>) -> Display {
        Display {}
    }
}
