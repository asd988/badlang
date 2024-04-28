use pest::error::Error;

use super::*;

impl CompiledCode {
    pub fn compile_str(mut self, from: &str) -> Result<CompiledCode, Error<Rule>> {
        let result = MyParser::parse(Rule::init, from)?;
    
        for pair in result {
            match pair.as_rule() {
                Rule::declaration => {
                    let simple_operation = get_simple_operation(pair);
                    if let Some(vars) = &mut self.variables {
                        vars.insert(simple_operation.identifier.clone());
                    }
                    self.instructions.push(Instruction::Declaration(simple_operation));
                },
                Rule::add => {
                    let simple_operation = get_simple_operation(pair);
                    self.instructions.push(Instruction::Add(simple_operation));
                },
                Rule::sub => {
                    let simple_operation = get_simple_operation(pair);
                    self.instructions.push(Instruction::Sub(simple_operation));
                },
                Rule::mul => {
                    let simple_operation = get_simple_operation(pair);
                    self.instructions.push(Instruction::Mul(simple_operation));
                },
                Rule::div => {
                    let simple_operation = get_simple_operation(pair);
                    self.instructions.push(Instruction::Div(simple_operation));
                },
                Rule::modulus => {
                    let simple_operation = get_simple_operation(pair);
                    self.instructions.push(Instruction::Mod(simple_operation));
                },
                Rule::max => {
                    let simple_operation = get_simple_operation(pair);
                    self.instructions.push(Instruction::Max(simple_operation));
                },
                Rule::min => {
                    let simple_operation = get_simple_operation(pair);
                    self.instructions.push(Instruction::Min(simple_operation));
                },
                Rule::invert => {
                    let identifier = pair.into_inner().next().unwrap().as_str().to_string();
                    self.instructions.push(Instruction::Invert(identifier));
                },
                Rule::delete => {
                    let identifier = pair.into_inner().next().unwrap().as_str().to_string();
                    self.instructions.push(Instruction::Delete(identifier));
                },
                Rule::print => {
                    let mut pairs: pest::iterators::Pairs<'_, Rule> = pair.into_inner();
                    let value = get_value(pairs.next().unwrap());
                    if let Some(text) = pairs.next() {
                        self.instructions.push(Instruction::Print(value, Some(text.as_str().to_string())));
                    } else {
                        self.instructions.push(Instruction::Print(value, None));
                    }
                },
                Rule::jump => {
                    let mut pairs = pair.into_inner();
                    let tag = pairs.next().unwrap().as_str().to_string();
                    if let Some(variable) = pairs.next() {
                        self.instructions.push(Instruction::Jump(tag, Some(get_value(variable))))
                    } else {                    
                        self.instructions.push(Instruction::Jump(tag, None));
                    }
                },
                Rule::tag => {
                    let tag = pair.into_inner().next().unwrap().as_str().to_string();
                    self.tags.insert(tag.clone(), Tag::Normal(self.instructions.len()));
                },
                Rule::stacked_tag => {
                    let tag = pair.into_inner().next().unwrap().as_str().to_string();
                    self.tags.insert(tag.clone(), Tag::Stacked(self.instructions.len()));
                },
                Rule::r#return => {
                    self.instructions.push(Instruction::Return);
                },
                _ => {}
            }
        }
        self.instructions.push(Instruction::Return);
    
        Ok(self)    
    }

    pub fn with_vars(mut self) -> Self {
        self.variables = Some(HashSet::new());
        self
    }
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
