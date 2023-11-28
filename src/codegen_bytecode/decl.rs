use crate::{codegen_bytecode::Generator, ir::hlir::Decl};

impl Generator<'_> {
    pub fn decl(&mut self, decl: &Decl) {
        match decl {
            Decl::Procedure { name, params, body } => {
                self.stmt(body);
            }
        }
    }
}
