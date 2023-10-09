use crate::codegen_c::{Generator, identifier};
use crate::parser::stmt::{Decl, Param};

impl Generator {
    pub fn decl(&mut self, decl: Decl) {
        match decl {
            Decl::Procedure {
                name, params, body
            } => {
                self.target.push_str(&format!("void {}(", identifier(name)));
                self.params(params);
                self.target.push(')');
                self.stmt(&body);
            }
        }
    }

    fn params(&mut self, params: Vec<Param>) {
        for param in params {
            self.type_name(param.type_name);
            self.target.push_str(&format!("{},", identifier(param.name)))
        }
        self.target.pop(); // remove trailing comma in arg list.
    }
}