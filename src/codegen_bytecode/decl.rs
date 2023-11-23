use crate::{codegen_bytecode::Generator, parser::stmt::Decl};

impl Generator {
    pub fn decl(&mut self, decl: &Decl) {
        match decl {
            Decl::Procedure { name, params, body } => {
                self.stmt(body);
            }
        }
    }
}