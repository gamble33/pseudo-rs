use crate::lexer::token::{KeywordKind, Token, TokenKind};
use crate::parser::error::ParseResult;
use crate::parser::expr::Expr;
use crate::parser::Parser;

pub enum Stmt {
    Expr(Expr),
}

impl<I> Parser<I>
where I: Iterator<Item = Token>
{
    pub fn stmt(&mut self) -> ParseResult<Stmt> {
        match self.tokens.peek() {
            Some(t) => match &t.kind {
                TokenKind::Keyword(keyword) => match keyword {
                    KeywordKind::Output => return self.output(),
                    _ => ()
                },
                _ => ()
            }
            None => todo!("Expected statement")
        }
        self.expr_stmt()
    }

    fn expr_stmt(&mut self) -> ParseResult<Stmt> {
        let expr_stmt = Stmt::Expr(self.expr()?);
        todo!()
    }

    fn output(&mut self) -> ParseResult<Stmt> {
        unimplemented!()
    }
}