use super::*;

pub struct AnalyserError {
    pub message: String,
    pub from_line: usize,
    pub from_col: usize,
    pub to_line: usize,
    pub to_col: usize
}


impl CompiledCode {
    pub fn analyse(&self) -> Result<(), Vec<AnalyserError>> {
        let mut errors = Vec::new();

        for instruction in self.instructions.iter() {
            if let Some(vars) = &self.variables {
                for id in instruction.get_variable_id().into_iter().filter_map(|x| x) {
                    if !vars.contains(&id.name) {
                        errors.push(AnalyserError {
                            message: format!("Variable {} not declared", id.name),
                            from_line: id.line,
                            from_col: id.col,
                            to_line: id.line,
                            to_col: id.col + id.name.len()
                        });
                    }

                }
            }

            if let Some(tag) = instruction.get_tag_id() {
                if !self.tags.contains_key(&tag.name) {
                    errors.push(AnalyserError {
                        message: format!("Tag {} not declared", tag.name),
                        from_line: tag.line,
                        from_col: tag.col,
                        to_line: tag.line,
                        to_col: tag.col + tag.name.len()
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