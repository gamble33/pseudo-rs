use crate::{naive_tc::TypeChecker, ir::ast::Decl};

impl TypeChecker {
    pub fn decl(&mut self, decl: &Decl) -> Decl {
        match decl {
            Decl::Procedure { name, params, body } => {
                self.stmt(body);
                todo!()
            }
        }
    }
}
