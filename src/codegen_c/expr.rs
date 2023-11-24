use crate::codegen_c::Generator;
use crate::codegen_c::identifier;
use crate::ir::ast::{Expr, LiteralKind};
use crate::lexer::token::{TokenKind, KeywordKind};

impl Generator {
    pub fn expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Binary { lhs, op, rhs } => {
                self.expr(lhs);
                match &op.kind {
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
            Expr::Logical { lhs, op, rhs } => {
                self.expr(lhs);
                match &op.kind {
                    TokenKind::Keyword(keyword) => match keyword {
                        KeywordKind::Or => self.target.push_str("|| "),
                        KeywordKind::And => self.target.push_str("&& "),
                        _ => unreachable!()
                    }
                    _ => unreachable!()
                }
                self.expr(rhs);
            },
            Expr::Unary { op, expr } => {
                match op.kind {
                    TokenKind::Minus => self.target.push_str("-"),
                    _ => unreachable!(),
                }
                self.expr(expr);
            },
            Expr::Assignment { target, value } => {
                self.expr(target);
                self.target.push_str("= ");
                self.expr(value);
            },
            Expr::Literal(literal) => {
                match literal {
                    LiteralKind::Integer(i) => self.target.push_str(&format!("{} ", i.to_string())),
                    LiteralKind::Character(ch) => self.target.push_str(&format!("(char)'{}'", *ch)),
                    LiteralKind::String(_) => todo!(),
                    LiteralKind::Boolean(boolean) => {
                        self.target.push_str("(bool)");
                        match boolean {
                            true => self.target.push_str("true"),
                            false => self.target.push_str("false"),
                        };
                    },
                }
            },
            Expr::Variable(name) => self.target.push_str(&format!("{} ", identifier(name))),
        }
    }
}
