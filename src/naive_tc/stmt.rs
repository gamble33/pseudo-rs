use crate::{naive_tc::TypeChecker, ir::{hlir, ast}};

impl TypeChecker {
    pub fn stmt(&mut self, stmt: ast::Stmt) -> hlir::Stmt {
        match stmt {
            ast::Stmt::If { condition, then_branch, else_branch } => unimplemented!(),
            ast::Stmt::Repeat { body, until } => unimplemented!(),
            ast::Stmt::While { body, condition } => unimplemented!(),
            ast::Stmt::Call { name, args } => unimplemented!(),
            ast::Stmt::VarDecl { name, type_name } => unimplemented!(),
            ast::Stmt::Expr(_) => unimplemented!(),
            ast::Stmt::Output(expr_kind) => hlir::Stmt::Output(self.expr(expr_kind)),
            ast::Stmt::Input(_) => unimplemented!(),
            ast::Stmt::Block(stmts) => hlir::Stmt::Block(
                    stmts.into_iter().map(|stmt| self.stmt(stmt)).collect()
            ),
        }
    }
}
