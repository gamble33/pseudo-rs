use super::Generator;
use crate::{
    ir::ast::LiteralKind,
    ir::hlir::{Expr, ExprKind},
    lexer::token::{KeywordKind, TokenKind::*},
    vm::{
        instr::Instr,
        obj::allocate_string,
        value::Value,
    },
};

impl Generator<'_> {
    pub fn expr(&mut self, expr: &Expr) {
        match &expr.expr_kind {
            ExprKind::Binary { lhs, op, rhs } => {
                self.expr(lhs);
                self.expr(rhs);
                match op.kind {
                    Ampersand => self.target.instructions.push(Instr::Concat),
                    Plus => self.target.instructions.push(Instr::Add(lhs.pseudo_type)),
                    Minus => self.target.instructions.push(Instr::Sub(lhs.pseudo_type)),
                    Star => self.target.instructions.push(Instr::Mul(lhs.pseudo_type)),
                    Slash => self.target.instructions.push(Instr::Div(lhs.pseudo_type)),
                    Greater => self.target.instructions.push(Instr::Gt(lhs.pseudo_type)),
                    GreaterEqual => self.target.instructions.push(Instr::GtEq(lhs.pseudo_type)),
                    Less => {
                        self.target.instructions.push(Instr::GtEq(lhs.pseudo_type));
                        self.target.instructions.push(Instr::Not);
                    }
                    LessEqual => {
                        self.target.instructions.push(Instr::Gt(lhs.pseudo_type));
                        self.target.instructions.push(Instr::Not);
                    }
                    Equal => self.target.instructions.push(Instr::Eq(lhs.pseudo_type)),
                    NotEqual => {
                        self.target.instructions.push(Instr::Eq(lhs.pseudo_type));
                        self.target.instructions.push(Instr::Not);
                    }
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
                LiteralKind::Integer(i) => self.emit_constant(Value {integer: *i}),
                LiteralKind::Boolean(b) => self.emit_constant(Value {boolean: *b}),
                LiteralKind::String(string) => {
                    let obj = allocate_string(self.vm, string.clone());
                    self.emit_constant(Value {obj});
                },
                LiteralKind::Character(ch) => self.emit_constant(Value {char: *ch}),
            },
            _ => unimplemented!(),
        }
    }

    fn emit_constant(&mut self, value: Value) {
        let instr = Instr::Const(self.target.add_constant(value));
        self.target.instructions.push(instr);
    }
}

