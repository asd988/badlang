use super::*;

impl Program {
    pub fn run(mut self) -> Program {
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
        let instruction = &self.code.instructions[ix];

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
                let tag = self.code.tags.get(tag).unwrap();
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