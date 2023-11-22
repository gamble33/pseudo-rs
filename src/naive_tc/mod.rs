mod decl;
mod stmt;

use std::collections::HashMap;
use crate::parser::stmt::Decl;

enum Type {

}

struct TypeChecker {
    symbol_table: HashMap<String, Type>
}

impl TypeChecker {
    fn typecheck(&mut self, decls: Vec<Decl>) -> Vec<Decl> {
        return decls.iter().map(|decl| self.decl(decl)).collect();
//            i love you silllyyyyyy
    }
}
