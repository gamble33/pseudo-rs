use crate::{ir::hlir::Stmt, vm::instr::Instr};

use super::Generator;

impl Generator<'_> {
    pub fn stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Output(expr) => {
                self.expr(expr);
                self.target.instructions.push(Instr::Output(expr.pseudo_type));
            },
            Stmt::If { condition, then_branch, else_branch } => todo!(),
            Stmt::Expr(expr) => {
                self.expr(expr);
                self.target.instructions.push(Instr::Pop);
            },
            Stmt::Call { name, args } => todo!(),
            Stmt::Input(_) => todo!(),
            Stmt::Block(stmts) => {
                self.enter_scope();
                stmts.iter().for_each(|stmt| self.stmt(stmt));
                self.exit_scope();
            },
            Stmt::While { body, condition } => todo!(),
            Stmt::Repeat { body, until } => todo!(),
            Stmt::VarDecl { name } => {
                self.target.instructions.push(Instr::Null);
                self.add_local(name.clone());
            },
        }
    }
}
