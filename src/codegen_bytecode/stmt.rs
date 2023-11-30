use super::Generator;
use crate::{ir::hlir::Stmt, vm::instr::Instr};

impl Generator<'_> {
    pub fn stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Output(expr) => {
                self.expr(expr);
                self.target.instructions.push(Instr::Output(expr.pseudo_type));
            },
            Stmt::If { condition, then_branch, else_branch } => {
                self.expr(condition);

                // emit jump instruction with meaningless value
                let jmp_false_idx = self.target.instructions.len();
                self.target.instructions.push(Instr::JumpFalse(0));

                self.target.instructions.push(Instr::Pop);
                self.stmt(then_branch);
                
                let jmp_else_idx = self.target.instructions.len();
                self.target.instructions.push(Instr::Jump(0));
                // todo: ELSE branching

                // patch jump instruction (now that its known where to jump to)
                self.target.instructions[jmp_false_idx] = Instr::JumpFalse(
                    self.target.instructions.len()
                );

                if let Some(else_branch) = else_branch {
                    self.stmt(else_branch);
                }

                self.target.instructions[jmp_else_idx] = Instr::Jump(
                    self.target.instructions.len()
                );
            },
            Stmt::Expr(expr) => {
                self.expr(expr);
                self.target.instructions.push(Instr::Pop);
            },
            Stmt::Call { name, args } => todo!(),
            Stmt::Input(holder) => {
                self.target.instructions.push(Instr::Input);
                let var_idx = self.resolve_local(holder);
                self.target.instructions.push(Instr::StoreLocal(var_idx));
                self.target.instructions.push(Instr::Pop);
            },
            Stmt::Block(stmts) => {
                self.enter_scope();
                stmts.iter().for_each(|stmt| self.stmt(stmt));
                self.exit_scope();
            },
            Stmt::While { body, condition } => {
                let loop_start_idx = self.target.instructions.len();
                self.expr(condition);
                let conditional_jmp_idx = self.target.instructions.len();
                self.target.instructions.push(Instr::JumpFalse(0));
                self.target.instructions.push(Instr::Pop);
                self.stmt(body);
                self.target.instructions.push(Instr::Jump(loop_start_idx));
                self.target.instructions[conditional_jmp_idx] = 
                    Instr::JumpFalse(self.target.instructions.len());
            },
            Stmt::Repeat { body, until } => {
                // Skip pop instruction on first iteration.
                let jmp_idx = self.target.instructions.len();
                self.target.instructions.push(Instr::Jump(0));

                let loop_start_idx = self.target.instructions.len();
                self.target.instructions.push(Instr::Pop);

                self.target.instructions[jmp_idx] =
                    Instr::Jump(self.target.instructions.len());

                self.stmt(body);
                self.expr(until);
                self.target.instructions.push(Instr::JumpFalse(loop_start_idx));
            },
            Stmt::VarDecl { name } => {
                self.target.instructions.push(Instr::Null);
                self.add_local(name.clone());
            },
        }
    }
}
