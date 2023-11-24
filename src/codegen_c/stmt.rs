use crate::codegen_c::{Generator, identifier};
use crate::ir::ast::Stmt;

impl Generator {
    pub fn stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::If { condition, then_branch, else_branch } => {
                self.target.push_str("if(");
                self.expr(condition);
                self.target.push(')');
                self.stmt(then_branch.as_ref());
                match else_branch {
                    Some(branch) => {
                        self.target.push_str("else");
                        self.stmt(branch);
                    }
                    _ => ()
                }
                
            }

            Stmt::Repeat { body, until } => {
                self.target.push_str("do ");
                self.stmt(body);
                self.target.push_str("while(!(");
                self.expr(until);
                self.target.push_str("));");
            },
            Stmt::While { body, condition } => {
                self.target.push_str("while(");
                self.expr(condition);
                self.target.push(')');
                self.stmt(body);
            },
            Stmt::Call { name, args } => {
                self.target.push_str(&identifier(name));
                self.target.push('(');
                if args.len() > 0 {
                    for arg in args {
                        self.expr(arg);
                        self.target.push(',');
                    }
                    self.target.pop();
                }
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
