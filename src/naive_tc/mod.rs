mod decl;

use std::collections::HashMap;
use crate::parser::stmt::Decl;

enum Type {

}

struct TypeChecker {
    symbol_table: HashMap<String, Type>
}

impl TypeChecker {
    fn do_ya_thang(decls: Vec<Decl>) -> Vec<Decl> {
        let mut tc = TypeChecker {
            symbol_table: HashMap::new(),
        };

        // todo: first pass for function and type declarations

        let type_checked_ast = decls.iter().map(|decl| tc.decl(decl));

        
//            i love you silllyyyyyy
        todo!()
    }
}