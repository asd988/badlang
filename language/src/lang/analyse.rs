use std::ptr;

use super::*;

pub struct AnalyserError {
    pub message: String,
    pub range: Range
}


impl CompiledCode {
    pub fn analyse(&mut self) -> Result<(), Vec<AnalyserError>> {
        let mut errors = Vec::new();

        for instruction in self.instructions.iter() {
            if let Some(vars) = &self.variables {
                for id in instruction.get_variable_id().into_iter().filter_map(|x| x) {
                    if !vars.contains(&id.name) {
                        errors.push(AnalyserError {
                            message: format!("Variable {} not declared", id.name),
                            range: id.range
                        });
                    }

                }
            }

            if let Some(tag) = instruction.get_tag_id() {
                if !self.tags.contains_key(&tag.name) {
                    errors.push(AnalyserError {
                        message: format!("Tag {} not declared", tag.name),
                        range: tag.range
                    });
                } else if let Some(locations) = &mut self.locations {
                    locations.push(TagLocation {
                        this: tag.range,
                        definition: ptr::addr_of!(self.tags[&tag.name]) as usize
                    });
                }
            }
        }
        
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}