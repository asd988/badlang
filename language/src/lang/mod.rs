use std::collections::{HashMap, HashSet};

use pest::{error::Error, Parser};
use pest_derive::Parser;

pub mod compile;
pub mod execute;
pub mod analyse;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MyParser;

#[derive(Default)]
pub struct CompiledCode {
    pub instructions: Vec<Instruction>,
    pub tags: HashMap<String, Tag>,
    pub variables: Option<HashSet<String>>,
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
    Identifier(Identifier),
    Number(i64)
}

impl Value {
    pub fn get_id(&self) -> Option<&Identifier> {
        match self {
            Value::Identifier(id) => Some(id),
            _ => None
        }
    }
}


#[derive(Debug)]
#[allow(dead_code)]
pub struct Identifier {
    name: String,
    line: usize,
    col: usize
}

impl Identifier {
    pub fn new(name: &str, line_col: (usize, usize)) -> Self {
        Identifier { name: name.to_string(), line: line_col.0, col: line_col.1 }
    }

    pub fn from_pair(pair: pest::iterators::Pair<Rule>) -> Self {
        Identifier::new(pair.as_str(), pair.line_col())
    }
}

#[derive(Debug)]
pub struct SimpleOperation {
    identifier: Identifier,
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
    Invert(Identifier),
    Delete(Identifier),
    Print(Value, Option<String>),
    Jump(Identifier, Option<Value>),
    Return
}

impl Instruction {
    pub fn get_tag_id(&self) -> Option<&Identifier> {
        match self {
            Instruction::Jump(tag, _) => Some(tag),
            _ => None
        }
    }

    pub fn get_variable_id(&self) -> [Option<&Identifier>; 2] {
        match self {
            Instruction::Declaration(simple_operation) => [Some(&simple_operation.identifier), simple_operation.value.get_id()],
            Instruction::Add(simple_operation) => [Some(&simple_operation.identifier), simple_operation.value.get_id()],
            Instruction::Sub(simple_operation) => [Some(&simple_operation.identifier), simple_operation.value.get_id()],
            Instruction::Mul(simple_operation) => [Some(&simple_operation.identifier), simple_operation.value.get_id()],
            Instruction::Div(simple_operation) => [Some(&simple_operation.identifier), simple_operation.value.get_id()],
            Instruction::Mod(simple_operation) => [Some(&simple_operation.identifier), simple_operation.value.get_id()],
            Instruction::Max(simple_operation) => [Some(&simple_operation.identifier), simple_operation.value.get_id()],
            Instruction::Min(simple_operation) => [Some(&simple_operation.identifier), simple_operation.value.get_id()],
            Instruction::Invert(identifier) => [Some(identifier), None],
            Instruction::Delete(identifier) => [Some(identifier), None],
            Instruction::Print(value, _) => [value.get_id(), None],
            Instruction::Jump(_tag, value) => [value.as_ref().and_then(|v| v.get_id()), None],
            _ => [None, None]
        }
    }
}