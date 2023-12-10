mod decl;
mod expr;
mod stmt;
mod types;
mod var;

use self::{
    decl::{define_decl, Callable},
    var::Variable,
};
use crate::ir::{ast, hlir::{self, Type}};
use std::collections::HashMap;

struct TypeChecker {
    symbol_table_stack: Vec<HashMap<String, Variable>>,
    callable_table: HashMap<String, Callable>,
    current_expected_return_type: Option<Type>,
}

pub fn typecheck(decls: Vec<ast::Decl>) -> Vec<hlir::Decl> {
    // First pass, declare all PROCEDUREs/FUNCTIONs
    let mut callable_table = HashMap::new();
    // todo: Don't clone the entire AST
    for decl in decls.clone().into_iter() {
        define_decl(decl, &mut callable_table);
    }

    if !callable_table.contains_key("Main") {
        unimplemented!("`PROCEDURE Main` wasn't defined");
    }

    let mut tc = TypeChecker {
        symbol_table_stack: vec![HashMap::new()],
        callable_table,
        current_expected_return_type: None,
    };

    let mut hlir_decls = Vec::new();
    for decl in decls.into_iter() {
        hlir_decls.push(tc.decl(decl));
    }
    hlir_decls
}

pub fn match_types(pseudo_type: &hlir::Type, types: &[hlir::Type]) -> bool {
    types.contains(pseudo_type)
}
