use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MyParser;

struct Program {
    instructions: Vec<Instruction>,
    variables: HashMap<String, i64>,
    tags: HashMap<String, Tag>,
    call_stack: Vec<usize>,
    next_instruction: usize,
}

enum Tag {
    Normal(usize),
    Stacked(usize)
}

enum Instruction {
    Assignment,
    Print,
}

fn main() {
    let result = MyParser::parse(Rule::init, "a = -10\na +s= 15\n< a").unwrap();

    for pair in result {
        println!("p {:?}", pair);
    }

}
