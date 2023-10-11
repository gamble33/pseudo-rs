use crate::codegen_c::Generator;
use crate::codegen_c::identifier;
use crate::parser::expr::{Expr, LiteralKind};
use crate::lexer::token::TokenKind;

impl Generator {
    pub fn expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Binary { lhs, op, rhs } => {
                self.expr(lhs);
                match op.kind {
                    TokenKind::Keyword(_) => todo!(),
                    TokenKind::Plus => self.target.push_str("+ "),
                    TokenKind::Minus => todo!(),
                    TokenKind::Slash => todo!(),
                    TokenKind::Star => todo!(),
                    TokenKind::Less => todo!(),
                    TokenKind::Greater => todo!(),
                    TokenKind::LessEqual => todo!(),
                    TokenKind::GreaterEqual => todo!(),
                    TokenKind::NotEqual => todo!(),
                    TokenKind::Equal => todo!(),
                    _ => unreachable!()
                }
                self.expr(rhs);
            },
            Expr::Logical { lhs, op, rhs } => todo!(),
            Expr::Unary { op, expr } => todo!(),
            Expr::Assignment { target, value } => {
                self.expr(target);
                self.target.push_str("= ");
                self.expr(value);
            },
            Expr::Literal(literal) => {
                match literal {
                    LiteralKind::Integer(i) => self.target.push_str(&format!("{} ", i.to_string())),
                    LiteralKind::Character(_) => todo!(),
                    LiteralKind::String(_) => todo!(),
                    LiteralKind::Boolean(_) => todo!(),
                }
            },
            Expr::Variable(name) => self.target.push_str(&format!("{} ", identifier(name))),
        }
    }
}