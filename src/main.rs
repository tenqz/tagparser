pub mod parser;
use crate::parser::Parser;
use std::env;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let html = &args[1];
    let tag = &args[2];

    let mut parser = Parser::new(html.to_string());
    println!("{:?}", parser.parse_tags(tag.to_string()));
}
