use super::Generator;
use crate::{
    ir::ast::LiteralKind,
    ir::hlir::{Expr, ExprKind},
    lexer::token::{TokenKind::*, KeywordKind},
    vm::{instr::Instr, value::Value},
};

impl Generator {
    fn emit_constant(&mut self, value: Value) {
        let index = self.target.add_constant(value);
        self.target.instructions.push(Instr::Const(index));
    }

    pub fn expr(&mut self, expr: &Expr) {
        match &expr.expr_kind {
            ExprKind::Binary { lhs, op, rhs } => {
                self.expr(lhs);
                self.expr(rhs);
                match op.kind {
                    Plus => self.target.instructions.push(Instr::Add(lhs.pseudo_type)),
                    Minus => self.target.instructions.push(Instr::Sub(lhs.pseudo_type)),
                    Star => self.target.instructions.push(Instr::Mul(lhs.pseudo_type)),
                    Slash => self.target.instructions.push(Instr::Div(lhs.pseudo_type)),
                    Greater => self.target.instructions.push(Instr::Gt(lhs.pseudo_type)),
                    GreaterEqual => self.target.instructions.push(Instr::GtEq(lhs.pseudo_type)),
                    Less => unimplemented!(),
                    LessEqual => unimplemented!(),
                    Equal => self.target.instructions.push(Instr::Eq(lhs.pseudo_type)),
                    NotEqual => unimplemented!(),
                    _ => unreachable!(),
                }
            }
            ExprKind::Unary { op, expr } => {
                self.expr(&expr);
                match op.kind {
                    Minus => self.target.instructions.push(Instr::Neg(expr.pseudo_type)),
                    Keyword(KeywordKind::Not) => self.target.instructions.push(Instr::Not),
                    _ => unreachable!(),
                }
            }
            ExprKind::Literal(literal) => match literal {
                LiteralKind::Integer(i) => self.emit_constant(Value { integer: *i }),
                LiteralKind::Boolean(b) => self.target.instructions.push(match b {
                    true => Instr::True,
                    false => Instr::False,
                }),
                LiteralKind::String(_) => unimplemented!(),
                LiteralKind::Character(_) => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }
}
