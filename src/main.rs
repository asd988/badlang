use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MyParser;



#[derive(Default)]
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

#[derive(Debug)]
enum Value {
    Identifier(String),
    Number(i64)
}

struct SimpleOperation {
    identifier: String,
    value: Value
}

enum Instruction {
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
    Print(Value),
    Jump(String),
    JumpIf(String, String),
    Return
}

fn main() {
    println!("asd");
    create_program("a = -10\na += 15\n< a").run();
}

fn create_program(from: &str) -> Program {
    let result = MyParser::parse(Rule::init, from).unwrap();

    let mut program = Program::default();

    let mut i = 1;

    for pair in result {
        match pair.as_rule() {
            Rule::declaration => {
                let simple_operation = get_simple_operation(pair);
                program.instructions.push(Instruction::Declaration(simple_operation));
            },
            Rule::add => {
                let simple_operation = get_simple_operation(pair);
                program.instructions.push(Instruction::Add(simple_operation));
            },
            Rule::sub => {
                let simple_operation = get_simple_operation(pair);
                program.instructions.push(Instruction::Sub(simple_operation));
            },
            Rule::mul => {
                let simple_operation = get_simple_operation(pair);
                program.instructions.push(Instruction::Mul(simple_operation));
            },
            Rule::div => {
                let simple_operation = get_simple_operation(pair);
                program.instructions.push(Instruction::Div(simple_operation));
            },
            Rule::modulus => {
                let simple_operation = get_simple_operation(pair);
                program.instructions.push(Instruction::Mod(simple_operation));
            },
            Rule::max => {
                let simple_operation = get_simple_operation(pair);
                program.instructions.push(Instruction::Max(simple_operation));
            },
            Rule::min => {
                let simple_operation = get_simple_operation(pair);
                program.instructions.push(Instruction::Min(simple_operation));
            },
            Rule::invert => {
                let identifier = pair.into_inner().next().unwrap().as_str().to_string();
                program.instructions.push(Instruction::Invert(identifier));
            },
            Rule::delete => {
                let identifier = pair.into_inner().next().unwrap().as_str().to_string();
                program.instructions.push(Instruction::Delete(identifier));
            },
            Rule::print => {
                let value = get_value(pair.into_inner().next().unwrap());
                program.instructions.push(Instruction::Print(value));
            },
            // TODO
            Rule::jump => {
                let tag = pair.into_inner().next().unwrap().as_str().to_string();
                program.instructions.push(Instruction::Jump(tag));
            },
            Rule::tag => {
                let tag = pair.into_inner().next().unwrap().as_str().to_string();
                program.tags.insert(tag.clone(), Tag::Normal(i));
            },
            Rule::stacked_tag => {
                let tag = pair.into_inner().next().unwrap().as_str().to_string();
                program.tags.insert(tag.clone(), Tag::Stacked(i));
            },
            Rule::r#return => {
                program.instructions.push(Instruction::Return);
            },
            _ => {}
        }
        i += 1;
    }
    program.instructions.push(Instruction::Return);

    program    
}

fn get_simple_operation(pair: pest::iterators::Pair<Rule>) -> SimpleOperation {
    let mut inner = pair.into_inner().into_iter();
    let identifier = inner.next().unwrap().as_str().to_string();
    let value = get_value(inner.next().unwrap());
    SimpleOperation { identifier, value }
}

fn get_value(pair: pest::iterators::Pair<Rule>) -> Value {
    match pair.as_rule() {
        Rule::identifier => Value::Identifier(pair.as_str().to_string()),
        Rule::number => Value::Number(pair.as_str().parse().unwrap()),
        _ => panic!("Unexpected value")
    }
}

impl Program {
    fn run(mut self) {
        loop {
            let current = self.next_instruction;
            self.next_instruction += 1;
            if self.execute_instrucion(current) {
                break;
            }
        }
    }

    fn execute_instrucion(&mut self, ix: usize) -> bool {
        let instruction = &self.instructions[ix];

        match instruction {
             Instruction::Declaration(simple_operation) => {
                self.variables.insert(simple_operation.identifier.clone(), self.get_from_value(&simple_operation.value));
            },
            Instruction::Add(simple_operation) => {
                let value = self.get_from_value(&simple_operation.value);
                let variable = self.variables.get_mut(&simple_operation.identifier).unwrap();
                *variable += value;
            },
            Instruction::Sub(simple_operation) => {
                let value = self.get_from_value(&simple_operation.value);
                let variable = self.variables.get_mut(&simple_operation.identifier).unwrap();
                *variable -= value;
            },
            Instruction::Mul(simple_operation) => {
                let value = self.get_from_value(&simple_operation.value);
                let variable = self.variables.get_mut(&simple_operation.identifier).unwrap();
                *variable *= value;
            },
            Instruction::Div(simple_operation) => {
                let value = self.get_from_value(&simple_operation.value);
                let variable = self.variables.get_mut(&simple_operation.identifier).unwrap();
                *variable /= value;
            },
            Instruction::Mod(simple_operation) => {
                let value = self.get_from_value(&simple_operation.value);
                let variable = self.variables.get_mut(&simple_operation.identifier).unwrap();
                *variable %= value;
            },
            Instruction::Max(simple_operation) => {
                let value = self.get_from_value(&simple_operation.value);
                let variable = self.variables.get_mut(&simple_operation.identifier).unwrap();
                *variable = std::cmp::max(*variable, value);
            },
            Instruction::Min(simple_operation) => {
                let value = self.get_from_value(&simple_operation.value);
                let variable = self.variables.get_mut(&simple_operation.identifier).unwrap();
                *variable = std::cmp::min(*variable, value);
            },
            Instruction::Invert(identifier) => {
                let variable = self.variables.get_mut(identifier).unwrap();
                *variable = if *variable == 0 { 1 } else { 0 };
            },
            Instruction::Delete(identifier) => {
                self.variables.remove(identifier);
            },
            Instruction::Print(value) => {
                println!("{}", self.get_from_value(value));
            },
            Instruction::Jump(tag) => {
                let tag = self.tags.get(tag).unwrap();
                match tag {
                    Tag::Normal(i) => {
                        self.next_instruction = *i;
                    },
                    Tag::Stacked(i) => {
                        self.call_stack.push(self.next_instruction);
                        self.next_instruction = *i;
                    }
                }
            },
            Instruction::JumpIf(_tag, _identifier) => {
                todo!()
            },
            Instruction::Return => {
                if let Some(i) = self.call_stack.pop() {
                    self.next_instruction = i;
                } else {
                    return true;
                }
            }
        }

        false
    }

    fn get_from_value(&self, value: &Value) -> i64 {
        match value {
            Value::Identifier(identifier) => {
                *self.variables.get(identifier).unwrap()
            },
            Value::Number(number) => *number
        }
    }
}