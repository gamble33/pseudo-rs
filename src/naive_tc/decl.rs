use crate::{naive_tc::TypeChecker, parser::stmt::Decl};

impl TypeChecker {
    pub fn decl(&mut self, decl: &Decl) -> Decl {
        match decl {
            Decl::Procedure { name, params, body } => todo!()
        }
    }
}