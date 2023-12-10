use crate::codegen_c::{identifier, Generator};
use crate::ir::ast::{Decl, Param};

impl Generator {
    pub fn decl(&mut self, decl: Decl) {
        match decl {
            Decl::Procedure { name, params, body } => {
                self.target.push_str(&format!("void {}", identifier(&name)));
                self.target.push('(');
                self.params(params);
                self.target.push(')');
                self.stmt(&body);
            },
            Decl::Function { .. } => unimplemented!()
        }
    }

    fn params(&mut self, params: Vec<Param>) {
        match params.len() {
            0 => self.target.push_str("void"),
            _ => {
                    for param in params {
                        self.type_name(&param.type_name);
                    self.target
                        .push_str(&format!("{},", identifier(&param.name)))
                }
                self.target.pop(); // remove trailing comma in arg list.
            }
        }
    }
}
