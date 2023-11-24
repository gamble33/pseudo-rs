use super::Generator;
use crate::{
    ir::ast::{Expr, LiteralKind},
    lexer::token::TokenKind::*,
    vm::{instr::Instr, value::Value}
};

impl Generator {

    fn emit_constant(&mut self, value: Value) {
        let index = self.target.add_constant(value);
        self.target.instructions.push(Instr::Const(index));
    }

    pub fn expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Binary { lhs, op, rhs } => {
                self.expr(lhs);
                self.expr(rhs);
                match op.kind {
                    Plus => self.target.instructions.push(Instr::Add),
                    Minus => self.target.instructions.push(Instr::Sub),
                    Star => self.target.instructions.push(Instr::Mul),
                    Slash => self.target.instructions.push(Instr::Div),
                    Greater => self.target.instructions.push(Instr::Gt),
                    GreaterEqual => self.target.instructions.push(Instr::GtEq),
                    Less => unimplemented!(),
                    LessEqual => unimplemented!(),
                    Equal => self.target.instructions.push(Instr::Eq),
                    NotEqual => unimplemented!(),
                    _ => unreachable!()
                }
            },
            Expr::Unary { op, expr } => {
                self.expr(expr);
                match op.kind {
                    Minus => self.target.instructions.push(Instr::Neg),
                    _ => unreachable!()
                }
            },
            Expr::Literal(literal) => match literal {
                LiteralKind::Integer(i) => self.emit_constant(*i),
                LiteralKind::Boolean(_) => unimplemented!(),
                LiteralKind::String(_) => unimplemented!(),
                LiteralKind::Character(_) => unimplemented!(),
            },
            _ => unimplemented!()
        }
    }
}
