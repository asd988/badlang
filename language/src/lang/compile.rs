use pest::error::Error;

use super::*;

impl CompiledCode {
    pub fn compile_str(mut self, from: &str) -> Result<CompiledCode, Error<Rule>> {
        let result = MyParser::parse(Rule::init, from)?;
    
        let mut doc = "".to_string();
        let mut last_doc_line = 0;

        for pair in result {
            let new_pair = pair.clone();
            match pair.as_rule() {
                Rule::declaration => {
                    let simple_operation = get_simple_operation(pair);
                    if let Some(vars) = &mut self.variables {
                        vars.insert(simple_operation.identifier.name.clone());
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
                    let identifier = pair.into_inner().next().unwrap();
                    self.instructions.push(Instruction::Invert(Identifier::from_pair(identifier)));
                },
                Rule::delete => {
                    let identifier = pair.into_inner().next().unwrap();
                    self.instructions.push(Instruction::Delete(Identifier::from_pair(identifier)));
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
                    let tag = pairs.next().unwrap();
                    if let Some(variable) = pairs.next() {
                        self.instructions.push(Instruction::Jump(Identifier::from_pair(tag), Some(get_value(variable))))
                    } else {                    
                        self.instructions.push(Instruction::Jump(Identifier::from_pair(tag), None));
                    }
                },
                Rule::tag => {
                    let tag = pair.into_inner().next().unwrap();
                    let (key, val) = Tag::from_pair(tag, TagType::Normal, self.instructions.len(), doc.clone());
                    self.tags.insert(key, val);
                },
                Rule::stacked_tag => {
                    let tag = pair.into_inner().next().unwrap();
                    let (key, val) = Tag::from_pair(tag, TagType::Stacked, self.instructions.len(), doc.clone());
                    self.tags.insert(key, val);
                },
                Rule::r#return => {
                    self.instructions.push(Instruction::Return);
                }
                _ => {}
            }
            let pair = new_pair;
            match pair.as_rule() {
                Rule::COMMENT => {
                    if last_doc_line != pair.line_col().0 - 1 {
                        doc = "".to_string();
                    }
                    last_doc_line = pair.line_col().0;
                    doc.push_str(pair.as_str());
                    doc.push_str("\n");
                },
                _ => {
                    doc = "".to_string();
                }
            }
        }
        self.instructions.push(Instruction::Return);
    
        Ok(self)    
    }

    pub fn lsp(mut self) -> Self {
        self.variables = Some(HashSet::new());
        self.locations = Some(Vec::new());
        self
    }
}

fn get_simple_operation(pair: pest::iterators::Pair<Rule>) -> SimpleOperation {
    let mut inner = pair.into_inner();
    let identifier = inner.next().unwrap();
    let value = get_value(inner.next().unwrap());
    SimpleOperation { identifier: Identifier::from_pair(identifier), value }
}

fn get_value(pair: pest::iterators::Pair<Rule>) -> Value {
    match pair.as_rule() {
        Rule::identifier => Value::Identifier(Identifier::from_pair(pair)),
        Rule::number => Value::Number(pair.as_str().parse().unwrap()),
        _ => panic!("Unexpected value")
    }
}
