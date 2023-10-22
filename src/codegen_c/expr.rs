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
                    TokenKind::Minus => self.target.push_str("- "),
                    TokenKind::Slash => self.target.push_str("/ "),
                    TokenKind::Star => self.target.push_str("* "),
                    TokenKind::Less => self.target.push_str("< "),
                    TokenKind::Greater => self.target.push_str("> "),
                    TokenKind::LessEqual => self.target.push_str("<= "),
                    TokenKind::GreaterEqual => self.target.push_str(">= "),
                    TokenKind::NotEqual => self.target.push_str("!= "),
                    TokenKind::Equal => self.target.push_str("== "),
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
