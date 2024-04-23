use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser;

#[cfg(test)]
mod test;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MyParser;

pub struct Program {
    instructions: Vec<Instruction>,
    variables: HashMap<String, i64>,
    tags: HashMap<String, Tag>,
    call_stack: Vec<usize>,
    next_instruction: usize,
    stdout_function: Box<dyn FnMut(String) -> ()>,
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
    fn with_stdout(stdout_function: impl FnMut(String) -> () + 'static) -> Self {
        Program {
            stdout_function: Box::new(stdout_function),
            ..Default::default()
        }
    }
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
    Print(Value, Option<String>),
    Jump(String, Option<Value>),
    Return
}

fn main() {
    // read "test" file
    let input = std::fs::read_to_string("test.txt").unwrap();

    //create_program("a = -10\na+= 15\n< a\n").run();

    let instant = std::time::Instant::now();
    let program = create_program(&input);
    println!("compilation: {:?}", instant.elapsed());
    let instant = std::time::Instant::now();
    program.run();
    println!("execution: {:?}", instant.elapsed());
}

pub fn create_program(from: &str) -> Program {
    compile_from_str(from, Program::default())
}

pub fn compile_from_str(from: &str, mut program: Program) -> Program {
    let result = MyParser::parse(Rule::init, from).unwrap();

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
                let mut pairs: pest::iterators::Pairs<'_, Rule> = pair.into_inner();
                let value = get_value(pairs.next().unwrap());
                if let Some(text) = pairs.next() {
                    program.instructions.push(Instruction::Print(value, Some(text.as_str().to_string())));
                } else {
                    program.instructions.push(Instruction::Print(value, None));
                }
            },
            Rule::jump => {
                let mut pairs = pair.into_inner();
                let tag = pairs.next().unwrap().as_str().to_string();
                if let Some(variable) = pairs.next() {
                    program.instructions.push(Instruction::Jump(tag, Some(get_value(variable))))
                } else {                    
                    program.instructions.push(Instruction::Jump(tag, None));
                }
            },
            Rule::tag => {
                let tag = pair.into_inner().next().unwrap().as_str().to_string();
                program.tags.insert(tag.clone(), Tag::Normal(program.instructions.len()));
            },
            Rule::stacked_tag => {
                let tag = pair.into_inner().next().unwrap().as_str().to_string();
                program.tags.insert(tag.clone(), Tag::Stacked(program.instructions.len()));
            },
            Rule::r#return => {
                program.instructions.push(Instruction::Return);
            },
            _ => {}
        }
    }
    program.instructions.push(Instruction::Return);

    program    
}

fn get_simple_operation(pair: pest::iterators::Pair<Rule>) -> SimpleOperation {
    let mut inner = pair.into_inner();
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
    fn run(mut self) -> Program {
        loop {
            let current = self.next_instruction;
            self.next_instruction += 1;
            if self.execute_instrucion(current) {
                break;
            }
        }
        self
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
            Instruction::Print(value, text) => {
                let value = self.get_from_value(value);
                if let Some(text) = text {
                    (self.stdout_function)(format!("{} {}", value, text));
                } else {
                    (self.stdout_function)(format!("{}", value));
                }
            },
            Instruction::Jump(tag, value) => {
                let tag = self.tags.get(tag).unwrap();
                if value.is_some() && self.get_from_value(value.as_ref().unwrap()) == 0 {
                    return false;
                }
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