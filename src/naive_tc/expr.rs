use super::TypeChecker;
use crate::ir::ast::{ExprKind, LiteralKind};
use crate::ir::hlir;

impl TypeChecker {
    pub fn expr(&mut self, expr: ExprKind) -> hlir::Expr {
        match expr {
            ExprKind::Binary { lhs, op, rhs } => {
                let lhs = self.expr(*lhs);
                let rhs = self.expr(*rhs);
                if lhs.pseudo_type != rhs.pseudo_type {
                    unimplemented!("Mismatched types on binary expr");
                }
                hlir::Expr {
                    pseudo_type: lhs.pseudo_type,
                    expr_kind: hlir::ExprKind::Binary {
                        lhs: Box::new(lhs),
                        op,
                        rhs: Box::new(rhs),
                    },
                }
            }
            ExprKind::Logical { lhs, op, rhs } => unimplemented!(),
            ExprKind::Unary { op, expr } => unimplemented!(),
            ExprKind::Assignment { target, value } => unimplemented!(),
            ExprKind::Literal(ref lit) => {
                let pseudo_type = match lit {
                    LiteralKind::Integer(_) => hlir::Type::Integer,
                    LiteralKind::Character(_) => hlir::Type::Char,
                    LiteralKind::String(_) => unimplemented!(),
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
