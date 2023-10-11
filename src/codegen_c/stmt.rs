use crate::codegen_c::{Generator, identifier};
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
            Stmt::Call { name, args } => {
                self.target.push_str(&identifier(name));
                self.target.push('(');
                for arg in args {
                    self.expr(arg);
                    self.target.push(',');
                }
                self.target.pop();
                self.target.push_str(");");
            },
            Stmt::VarDecl { name, type_name } => {
                self.type_name(type_name);
                self.target.push_str(&identifier(name));
                self.target.push(';');
            },
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