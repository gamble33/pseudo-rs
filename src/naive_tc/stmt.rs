use super::types::pseudo_type;
use crate::{
    ir::{ast, hlir},
    naive_tc::TypeChecker,
};

impl TypeChecker {
    pub fn stmt(&mut self, stmt: ast::Stmt) -> hlir::Stmt {
        match stmt {
            ast::Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let condition = self.expr(condition);
                if condition.pseudo_type != hlir::Type::Boolean {
                    unimplemented!("IF condition must be of BOOLEAN type.");
                }
                hlir::Stmt::If {
                    condition,
                    then_branch: Box::new(self.stmt(*then_branch)),
                    else_branch: match else_branch {
                        Some(else_branch) => Some(Box::new(self.stmt(*else_branch))),
                        None => None,
                    },
                }
            }
            ast::Stmt::Repeat { body, until } => {
                let body = Box::new(self.stmt(*body));
                let until = self.expr(until);
                hlir::Stmt::Repeat { body, until }
            }
            ast::Stmt::While { body, condition } => {
                let body = Box::new(self.stmt(*body));
                let condition = self.expr(condition);
                hlir::Stmt::While { body, condition }
            }
            ast::Stmt::Call { name, args } => {
                if !self.check_decl_exists(&name) {
                    unimplemented!("CALL to undefined PROCEDURE `{}`.", name);
                }
                let args: Vec<hlir::Expr> = args.into_iter().map(|arg| self.expr(arg)).collect();
                if let Some(procedure) = self.callable_table.get(&name) {
                    if args.len() != procedure.params.len() {
                        unimplemented!("wrong number of arguments");
                    }
                    for (param, arg) in procedure.params.iter().zip(args.iter()) {
                        if param.pseudo_type != arg.pseudo_type {
                            unimplemented!("wrong Type of argument");
                        }
                    }
                }
                hlir::Stmt::Call { name, args }
            }
            ast::Stmt::VarDecl { name, type_name } => {
                let pseudo_type = pseudo_type(&type_name);
                self.decl_var(name.clone(), pseudo_type);
                hlir::Stmt::VarDecl { name }
            }
            ast::Stmt::Expr(expr_kind) => hlir::Stmt::Expr(self.expr(expr_kind)),
            ast::Stmt::Output(expr_kind) => hlir::Stmt::Output(self.expr(expr_kind)),
            ast::Stmt::Input(holder) => {
                let var = match self.get_var_mut(&holder) {
                    Some(var) => var,
                    None => unimplemented!("invalid assignment target for input buf"),
                };
                var.initialized = true;
                hlir::Stmt::Input(holder)
            }
            ast::Stmt::Block(stmts) => {
                self.enter_scope();
                let block =
                    hlir::Stmt::Block(stmts.into_iter().map(|stmt| self.stmt(stmt)).collect());
                self.exit_scope();
                block
            }
        }
    }
}
