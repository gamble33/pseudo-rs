use super::Generator;
use crate::{
    ir::ast::LiteralKind,
    ir::hlir::{Expr, ExprKind},
    lexer::token::{KeywordKind, TokenKind::*},
    vm::{instr::Instr, value::Value, obj::{Obj, ObjString, ObjKind}},
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
                LiteralKind::Integer(i) => self.emit_constant(Value { integer: *i }),
                LiteralKind::Boolean(b) => self.target.instructions.push(match b {
                    true => Instr::True,
                    false => Instr::False,
                }),
                LiteralKind::String(string) => self.emit_constant(Value {
                    obj: Box::into_raw(Box::new(ObjString {
                        obj: Obj { kind: ObjKind::String },
                        string: string.clone(),
                    })) as *const Obj
                }) ,
                LiteralKind::Character(ch) => self.emit_constant(Value {char: *ch}),
            },
            _ => unimplemented!(),
        }
    }
}
