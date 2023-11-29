use super::Generator;
use crate::vm::instr::Instr;

pub struct Local {
    name: String,
    depth: u8,
}

impl Generator<'_> {
    pub fn enter_scope(&mut self) {
        self.scope_depth += 1;
    }

    pub fn exit_scope(&mut self) {
        self.scope_depth -= 1;
        while self
            .locals
            .last()
            .is_some_and(|local| local.depth > self.scope_depth)
        {
            self.target.instructions.push(Instr::Pop);
            self.locals.pop();
        }
    }

    pub fn add_local(&mut self, name: String) {
        self.locals.push(Local {
            name,
            depth: self.scope_depth,
        });
    }

    pub fn resolve_local(&mut self, name: &str) -> usize {
        self.locals
            .iter()
            .enumerate()
            .rev()
            .find_map(|(idx, local)| {
                if local.name == name {
                    Some(idx)
                } else {
                    None
                }
            }).unwrap()
    }
}
