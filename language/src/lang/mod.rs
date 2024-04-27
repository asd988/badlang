use std::collections::HashMap;

use pest::{error::Error, Parser};
use pest_derive::Parser;

pub mod compile;
pub mod execute;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MyParser;

#[derive(Default)]
pub struct CompiledCode {
    pub instructions: Vec<Instruction>,
    pub tags: HashMap<String, Tag>
}

pub struct Program {
    pub code: CompiledCode,
    variables: HashMap<String, i64>,
    call_stack: Vec<usize>,
    next_instruction: usize,
    pub stdout_function: Box<dyn FnMut(String) -> ()>,
}

impl Default for Program {
    fn default() -> Self {
        Program {
            code: CompiledCode::default(),
            variables: HashMap::new(),
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

    pub fn compile_str(mut self, from: &str) -> Result<Self, Error<Rule>> {
        self.code = self.code.compile_str(from)?;
        Ok(self)
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