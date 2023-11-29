use super::TypeChecker;
use crate::ir::hlir::Type;
use std::collections::HashMap;

pub struct Variable {
    pub pseudo_type: Type,
    pub initialized: bool,
}

impl TypeChecker {
    pub fn enter_scope(&mut self) {
        self.symbol_table_stack.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.symbol_table_stack.pop();
    }

    pub fn decl_var(&mut self, name: String, pseudo_type: Type) {
        if self.check_var_exists(&name) {
            panic!("var `{}` already declared", name);
        }
        self.symbol_table_stack.last_mut().unwrap().insert(
            name,
            Variable {
                pseudo_type,
                initialized: false,
            },
        );
    }

    pub fn get_var_mut(&mut self, name: &str) -> Option<&mut Variable> {
        match self
            .symbol_table_stack
            .iter_mut()
            .rev()
            .find(|symbol_table| symbol_table.contains_key(name))
        {
            Some(symbol_table) => symbol_table.get_mut(name),
            None => None,
        }
    }

    fn check_var_exists(&self, name: &str) -> bool {
        self.symbol_table_stack
            .iter()
            .rev()
            .find(|symbol_table| symbol_table.contains_key(name))
            .is_some()
    }
}
