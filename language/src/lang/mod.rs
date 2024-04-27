use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;

pub mod compile;
pub mod execute;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MyParser;

pub struct Program {
    pub instructions: Vec<Instruction>,
    variables: HashMap<String, i64>,
    tags: HashMap<String, Tag>,
    call_stack: Vec<usize>,
    next_instruction: usize,
    pub stdout_function: Box<dyn FnMut(String) -> ()>,
}

impl Default for Program {
    fn default() -> Self {
        Program {
            instructions: Vec::new(),
            variables: HashMap::new(),
            tags: HashMap::new(),
            call_stack: Vec::new(),
            next_instruction: 0,
            stdout_function: Box::new(|text| println!("{}", text))
        }
    }
}

impl Program {
    #[allow(unused)]
    pub fn with_stdout(stdout_function: impl FnMut(String) -> () + 'static) -> Self {
        Program {
            stdout_function: Box::new(stdout_function),
            ..Default::default()
        }
    }
}

pub enum Tag {
    Normal(usize),
    Stacked(usize)
}

#[derive(Debug)]
pub enum Value {
    Identifier(String),
    Number(i64)
}

#[derive(Debug)]
pub struct SimpleOperation {
    identifier: String,
    value: Value
}

#[derive(Debug)]
pub enum Instruction {
    Declaration(SimpleOperation),
    Add(SimpleOperation),
    Sub(SimpleOperation),
    Min(SimpleOperation),
    Mul(SimpleOperation),
    Div(SimpleOperation),
    Mod(SimpleOperation),
    Max(SimpleOperation),
    Invert(String),
    Delete(String),
    Print(Value, Option<String>),
    Jump(String, Option<Value>),
    Return
}