use crate::{naive_tc::TypeChecker, ir::{ast, hlir}};

impl TypeChecker {
    pub fn decl(&mut self, decl: ast::Decl) -> hlir::Decl {
        match decl {
            ast::Decl::Procedure { name, params, body } => {
                hlir::Decl::Procedure {
                    name,
                    params: self.params(params),
                    body: self.stmt(body),
                }
            }
        }
    }

    pub fn params(&mut self, params: Vec<ast::Param>) -> Vec<hlir::Param> {
        params.into_iter().map(|param| hlir::Param {
            name: param.name,
            pseudo_type: self.pseudo_type(param.type_name),
            passing_mode: param.passing_mode,
        }).collect()
    }
}
