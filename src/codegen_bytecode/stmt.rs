use crate::{parser::stmt::Stmt, vm::instr::Instr};

use super::Generator;

impl Generator {
    pub fn stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Output(expr) => {
                self.expr(expr);
                self.target.instructions.push(Instr::Output);
            },
            Stmt::If { condition, then_branch, else_branch } => todo!(),
            Stmt::Expr(_) => todo!(),
            Stmt::Call { name, args } => todo!(),
            Stmt::Input(_) => todo!(),
            Stmt::Block(stmts) => {
                stmts.iter().for_each(|stmt| self.stmt(stmt));
            },
            Stmt::While { body, condition } => todo!(),
            Stmt::Repeat { body, until } => todo!(),
            Stmt::VarDecl { name, type_name } => todo!(),
        }
    }
}
