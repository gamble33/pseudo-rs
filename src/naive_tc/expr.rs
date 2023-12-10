use super::{match_types, TypeChecker};
use crate::ir::ast::{self, LiteralKind};
use crate::ir::hlir::{self, Type};
use crate::lexer::token::{KeywordKind, TokenKind};

impl TypeChecker {
    pub fn expr(&mut self, expr: ast::ExprKind) -> hlir::Expr {
        match expr {
            ast::ExprKind::Binary { lhs, op, rhs } => {
                use TokenKind::*;
                let lhs = self.expr(*lhs);
                let rhs = self.expr(*rhs);
                if lhs.pseudo_type != rhs.pseudo_type {
                    unimplemented!("Mismatched types on binary expr");
                }
                let pseudo_type = match op.kind {
                    Greater | GreaterEqual | Less | LessEqual => {
                        if !match_types(&lhs.pseudo_type, &[Type::Real, Type::Integer]) {
                            unimplemented!(
                                "Cannot do comparison of anything other than INTEGER or REAL"
                            );
                        }
                        Type::Boolean
                    }
                    Equal | NotEqual => Type::Boolean,
                    Ampersand => {
                        if lhs.pseudo_type != Type::String {
                            unimplemented!("Can only concatenate two strings");
                        }
                        Type::String
                    }
                    _ => lhs.pseudo_type,
                };
                hlir::Expr {
                    pseudo_type,
                    expr_kind: hlir::ExprKind::Binary {
                        lhs: Box::new(lhs),
                        op,
                        rhs: Box::new(rhs),
                    },
                }
            }
            ast::ExprKind::Logical { lhs, op, rhs } => {
                let lhs = self.expr(*lhs);
                let rhs = self.expr(*rhs);
                if lhs.pseudo_type != Type::Boolean || rhs.pseudo_type != Type::Boolean {
                    unimplemented!(
                        "Logical comparison must be done on expressions of BOOLEAN type."
                    );
                }
                hlir::Expr {
                    pseudo_type: Type::Boolean,
                    expr_kind: hlir::ExprKind::Logical {
                        lhs: Box::new(lhs),
                        op,
                        rhs: Box::new(rhs),
                    },
                }
            }
            ast::ExprKind::Unary { op, expr } => {
                let expr = self.expr(*expr);
                match op.kind {
                    TokenKind::Keyword(KeywordKind::Not) => {
                        if expr.pseudo_type != Type::Boolean {
                            unimplemented!("Can only do NOT operation on booleans");
                        }
                    }
                    TokenKind::Minus => {
                        if !match_types(&expr.pseudo_type, &[Type::Real, Type::Integer]) {
                            unimplemented!("Can only negate REALs & INTEGERs");
                        }
                    }
                    _ => unreachable!(),
                };
                hlir::Expr {
                    pseudo_type: expr.pseudo_type,
                    expr_kind: hlir::ExprKind::Unary {
                        op,
                        expr: Box::new(expr),
                    },
                }
            }
            ast::ExprKind::Assignment { target, value } => {
                let value = self.expr(*value);
                let var_target = match self.get_var_mut(&target) {
                    Some(var) => var,
                    None => unimplemented!("Attempting to assign to an undeclared variable"),
                };
                if var_target.pseudo_type != value.pseudo_type {
                    unimplemented!("Cannot assign type A to type B")
                }
                var_target.initialized = true;
                hlir::Expr {
                    pseudo_type: var_target.pseudo_type,
                    expr_kind: hlir::ExprKind::Assignment {
                        target,
                        value: Box::new(value),
                    },
                }
            }
            ast::ExprKind::Call { callee, args } => {
                let callee = match *callee {
                    ast::ExprKind::Variable(name) => name,
                    _ => unimplemented!("Invalid FUCNTION callee expression"),
                };
                let args: Vec<hlir::Expr> = args.into_iter().map(|arg| self.expr(arg)).collect();

                if let Some(function) = self.callable_table.get(&callee) {
                    if args.len() != function.params.len() {
                        unimplemented!("Wrong number of arguments");
                    }
                    for (param, arg) in function.params.iter().zip(args.iter()) {
                        if param.pseudo_type != arg.pseudo_type {
                            unimplemented!("Wrong type of argument");
                        }
                    }
                    hlir::Expr {
                        pseudo_type: function.return_type.unwrap(),
                        expr_kind: hlir::ExprKind::Call { callee, args },
                    }
                } else {
                    unimplemented!("Call to undefined FUNCTION `{}`", callee);
                }
            }
            ast::ExprKind::Literal(ref lit) => {
                let pseudo_type = match lit {
                    LiteralKind::Integer(_) => hlir::Type::Integer,
                    LiteralKind::Real(_) => hlir::Type::Real,
                    LiteralKind::Character(_) => hlir::Type::Char,
                    LiteralKind::String(_) => hlir::Type::String,
                    LiteralKind::Boolean(_) => hlir::Type::Boolean,
                };
                hlir::Expr {
                    pseudo_type,
                    expr_kind: hlir::ExprKind::Literal(lit.clone()),
                }
            }
            ast::ExprKind::Variable(name) => {
                let var = match self.get_var_mut(&name) {
                    Some(var) => var,
                    None => unimplemented!("variable `{}` not declated", name),
                };
                if !var.initialized {
                    unimplemented!("use of uninitialized variable `{}`.", name);
                }
                hlir::Expr {
                    pseudo_type: var.pseudo_type,
                    expr_kind: hlir::ExprKind::Variable(name),
                }
            }
        }
    }
}
