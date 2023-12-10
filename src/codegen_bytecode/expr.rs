use super::Generator;
use crate::{
    ir::ast::LiteralKind,
    ir::hlir::{Expr, ExprKind},
    lexer::token::{KeywordKind, TokenKind::*},
    vm::{instr::Instr, obj::allocate_string, value::Value},
};

impl Generator<'_> {
    pub fn expr(&mut self, expr: &Expr) {
        match &expr.expr_kind {
            ExprKind::Binary { lhs, op, rhs } => {
                self.expr(&lhs);
                self.expr(&rhs);
                match op.kind {
                    Ampersand => self.emit(Instr::Concat),
                    Plus => self.emit(Instr::Add(lhs.pseudo_type)),
                    Minus => self.emit(Instr::Sub(lhs.pseudo_type)),
                    Star => self.emit(Instr::Mul(lhs.pseudo_type)),
                    Slash => self.emit(Instr::Div(lhs.pseudo_type)),
                    Greater => self.emit(Instr::Gt(lhs.pseudo_type)),
                    GreaterEqual => self.emit(Instr::GtEq(lhs.pseudo_type)),
                    Less => {
                        self.emit(Instr::GtEq(lhs.pseudo_type));
                        self.emit(Instr::Not);
                    }
                    LessEqual => {
                        self.emit(Instr::Gt(lhs.pseudo_type));
                        self.emit(Instr::Not);
                    }
                    Equal => self.emit(Instr::Eq(lhs.pseudo_type)),
                    NotEqual => {
                        self.emit(Instr::Eq(lhs.pseudo_type));
                        self.emit(Instr::Not);
                    }
                    _ => unreachable!(),
                }
            }
            ExprKind::Unary { op, expr } => {
                self.expr(&expr);
                match op.kind {
                    Minus => self.emit(Instr::Neg(expr.pseudo_type)),
                    Keyword(KeywordKind::Not) => self.emit(Instr::Not),
                    _ => unreachable!(),
                }
            }
            ExprKind::Literal(literal) => match literal {
                LiteralKind::Integer(i) => self.emit_constant(Value { integer: *i }),
                LiteralKind::Boolean(b) => self.emit_constant(Value { boolean: *b }),
                LiteralKind::String(string) => {
                    let obj = allocate_string(self.vm, string.clone());
                    self.emit_constant(Value { obj });
                }
                LiteralKind::Character(ch) => self.emit_constant(Value { char: *ch }),
            },
            ExprKind::Variable(name) => {
                let arg = self.resolve_local(name);
                self.emit(Instr::LoadLocal(arg));
            }
            ExprKind::Assignment { target, value } => {
                self.expr(&value);
                let arg = self.resolve_local(target);
                self.emit(Instr::StoreLocal(arg));
            }
            ExprKind::Logical { lhs, op, rhs } => {
                self.expr(&lhs);

                match &op.kind {
                    Keyword(keyword) => match keyword {
                        KeywordKind::Or => {
                            let jmp_idx = self.target().instructions.len();
                            self.emit(Instr::JumpTrue(0));
                            self.emit(Instr::Pop);
                            self.expr(&rhs);
                            self.target().instructions[jmp_idx] =
                                Instr::JumpTrue(self.target().instructions.len());
                        },
                        KeywordKind::And => {
                            let jmp_idx = self.target().instructions.len();
                            self.emit(Instr::JumpFalse(0));
                            self.emit(Instr::Pop);
                            self.expr(&rhs);
                            self.target().instructions[jmp_idx] =
                                Instr::JumpFalse(self.target().instructions.len());
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            }
        }
    }
}
