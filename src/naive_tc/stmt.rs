use crate::{naive_tc::TypeChecker, ir::{hlir, ast}};

impl TypeChecker {
    pub fn stmt(&mut self, stmt: ast::Stmt) -> hlir::Stmt {
        match stmt {
            ast::Stmt::If { condition, then_branch, else_branch } => unimplemented!(),
            ast::Stmt::Repeat { body, until } => unimplemented!(),
            ast::Stmt::While { body, condition } => unimplemented!(),
            ast::Stmt::Call { name, args } => unimplemented!(),
            ast::Stmt::VarDecl { name, type_name } => {
                let pseudo_type = self.pseudo_type(type_name);
                self.decl_var(name.clone(), pseudo_type);
                hlir::Stmt::VarDecl { name }
            }
            ast::Stmt::Expr(expr_kind) => hlir::Stmt::Expr(self.expr(expr_kind)),
            ast::Stmt::Output(expr_kind) => hlir::Stmt::Output(self.expr(expr_kind)),
            ast::Stmt::Input(_) => unimplemented!(),
            ast::Stmt::Block(stmts) => {
                self.enter_scope();
                let block = hlir::Stmt::Block(
                    stmts.into_iter().map(|stmt| self.stmt(stmt)).collect()
                );
                self.exit_scope();
                block
            }
        }
    }
}
