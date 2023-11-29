mod decl;
mod stmt;
mod expr;
mod types;
mod var;

use self::var::Variable;
use crate::ir::{hlir, ast};
use std::collections::HashMap;

struct TypeChecker {
    symbol_table_stack: Vec<HashMap<String, Variable>>,
}

pub fn typecheck(decls: Vec<ast::Decl>) -> Vec<hlir::Decl> {
    let mut tc = TypeChecker {
        symbol_table_stack: vec![HashMap::new()],
    };
    decls.into_iter().map(|decl| tc.decl(decl)).collect()
}

pub fn match_types(pseudo_type: &hlir::Type, types: &[hlir::Type]) -> bool {
    types.contains(pseudo_type)
}
