use std::collections::HashMap;

use crate::{
    ir::{ast, hlir},
    naive_tc::TypeChecker,
};

use super::types::pseudo_type;

enum CallableKind {
    Procedure,
    Function,
}

pub struct Callable {
    kind: CallableKind,
    pub params: Vec<IrParam>,
    return_type: Option<hlir::Type>,
}

pub struct IrParam {
    name: String,
    pub pseudo_type: hlir::Type,
    passing_mode: Option<ast::PassingMode>,
}

pub fn define_decl(decl: ast::Decl, map: &mut HashMap<String, Callable>) {
    match decl {
        ast::Decl::Procedure { name, params, body } => {
            if map.contains_key(&name) {
                unimplemented!("Procedure defined twice.");
            }
            let callable = Callable {
                kind: CallableKind::Procedure,
                params: params
                    .iter()
                    .map(|param| IrParam {
                        name: name.clone(),
                        pseudo_type: pseudo_type(&param.type_name),
                        passing_mode: param.passing_mode,
                    })
                    .collect(),
                return_type: None,
            };
            map.insert(name, callable);
        }
    }
}

impl TypeChecker {
    pub fn decl(&mut self, decl: ast::Decl) -> hlir::Decl {
        match decl {
            ast::Decl::Procedure { name, params, body } => {
                self.enter_scope();
                let params = self.params(params);
                self.declare_params(&params);
                let procedure = hlir::Decl::Procedure {
                    name,
                    params,
                    body: self.stmt(body),
                };
                self.exit_scope();
                procedure
            }
        }
    }

    pub fn check_decl_exists(&self, name: &String) -> bool {
        self.callable_table.contains_key(name)
    }

    pub fn params(&mut self, params: Vec<ast::Param>) -> Vec<hlir::Param> {
        params
            .into_iter()
            .map(|param| hlir::Param {
                name: param.name,
                pseudo_type: pseudo_type(&param.type_name),
                passing_mode: param.passing_mode,
            })
            .collect()
    }

    fn declare_params(&mut self, params: &Vec<hlir::Param>) {
        params.iter().for_each(|param| {
            self.decl_var(param.name.clone(), param.pseudo_type);
            self.symbol_table_stack
                .last_mut()
                .unwrap()
                .get_mut(&param.name)
                .unwrap()
                .initialized = true;
        });
    }
}
