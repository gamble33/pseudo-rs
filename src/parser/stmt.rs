use crate::lexer::token::{KeywordKind, Token, TokenKind};
use crate::parser::error::ParseResult;
use crate::parser::expr::Expr;
use crate::parser::Parser;

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Output(Expr),
}

impl<I> Parser<I>
    where I: Iterator<Item=Token>
{
    pub fn declaration(&mut self) -> ParseResult<Stmt> {
        match self.tokens.peek() {
            Some(t) => match &t.kind {
                TokenKind::Keyword(keyword) => match keyword {
                    KeywordKind::Declare => return self.var_declaration(),
                    _ => ()
                },
                _ => ()
            }
            None => ()
        }
        self.stmt()
    }

    fn var_declaration(&mut self) -> ParseResult<Stmt> {
        self.tokens.next();
        todo!()
    }

    fn stmt(&mut self) -> ParseResult<Stmt> {
        match self.tokens.peek() {
            Some(t) => match &t.kind {
                TokenKind::Keyword(keyword) => match keyword {
                    KeywordKind::Output => return self.output(),
                    _ => ()
                },
                _ => ()
            }
            None => return self.error(
                String::from("expected statement"),
                None
            )
        }
        self.expr_stmt()
    }

    fn expr_stmt(&mut self) -> ParseResult<Stmt> {
        let expr_stmt = Stmt::Expr(self.expr()?);
        self.consume(
            TokenKind::NewLine,
            String::from("expected new line after expression."),
        )?;
        Ok(expr_stmt)
    }

    fn output(&mut self) -> ParseResult<Stmt> {
        self.tokens.next();
        let expr = self.expr()?;
        self.consume(
            TokenKind::NewLine,
            String::from("expected new line after expression."),
        )?;
        Ok(Stmt::Output(expr))
    }
}