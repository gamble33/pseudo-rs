use crate::codegen_c::Generator;
use crate::parser::stmt::Stmt;

impl Generator {
    pub fn stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::If { condition, then_branch, else_branch } => {
                self.target.push_str("if(");
                self.expr(condition);
                self.target.push(')');
                self.stmt(then_branch.as_ref());
                self.target.push_str("else");
                self.stmt(then_branch.as_ref());
            }

            Stmt::Repeat { .. } => unimplemented!(),
            Stmt::While { .. } => unimplemented!(),
            Stmt::Call { .. } => unimplemented!(),
            Stmt::VarDecl { .. } => unimplemented!(),
            Stmt::Expr(expr) => {
                self.expr(expr);
                self.target.push(';');
            }
            Stmt::Output(expr) => {
                self.target.push_str("print(");
                self.expr(expr);
                self.target.push_str(");");
            }
            Stmt::Input(_) => unimplemented!(),
            Stmt::Block(stmts) => {
                self.target.push('{');
                for stmt in stmts {
                    self.stmt(stmt);
                }
                self.target.push('}');
            }
        }
    }
}