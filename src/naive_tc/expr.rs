use super::{TypeChecker, match_types};
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
                            unimplemented!("Cannot do comparison of anything other than INTEGER or REAL");
                        }
                        Type::Boolean
                    },
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
                    pseudo_type: pseudo_type,
                    expr_kind: hlir::ExprKind::Binary {
                        lhs: Box::new(lhs),
                        op,
                        rhs: Box::new(rhs),
                    },
                }
            }
            ExprKind::Logical { lhs, op, rhs } => unimplemented!(),
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
            ExprKind::Assignment { target, value } => unimplemented!(),
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
            ExprKind::Variable(_) => unimplemented!(),
        }
    }
}
