use super::{match_types, TypeChecker};
use crate::ir::ast::{ExprKind, LiteralKind};
use crate::ir::hlir::{self, Type};
use crate::lexer::token::{KeywordKind, TokenKind};

impl TypeChecker {
    pub fn expr(&mut self, expr: ExprKind) -> hlir::Expr {
        match expr {
            ExprKind::Binary { lhs, op, rhs } => {
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
            ExprKind::Logical { lhs, op, rhs } => {
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
            ExprKind::Unary { op, expr } => {
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
            ExprKind::Assignment { target, value } => {
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
            ExprKind::Literal(ref lit) => {
                let pseudo_type = match lit {
                    LiteralKind::Integer(_) => hlir::Type::Integer,
                    LiteralKind::Character(_) => hlir::Type::Char,
                    LiteralKind::String(_) => hlir::Type::String,
                    LiteralKind::Boolean(_) => hlir::Type::Boolean,
                };
                hlir::Expr {
                    pseudo_type,
                    expr_kind: hlir::ExprKind::Literal(lit.clone()),
                }
            }
            ExprKind::Variable(name) => {
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
