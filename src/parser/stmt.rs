use crate::lexer::token::{KeywordKind, Token, TokenKind};
use crate::parser::error::ParseResult;
use crate::parser::expr::Expr;
use crate::parser::Parser;

#[derive(Debug)]
pub enum Stmt {
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },

    Repeat {
        body: Box<Stmt>,
        until: Expr
    },

    While {
        body: Box<Stmt>,
        condition: Expr,
    },

    For {
        initializer: Expr,
        to: Expr,
        step: Option<Expr>,
        body: Box<Stmt>,
        counter: Expr,

    },

    Expr(Expr),
    Output(Expr),
    Block(Vec<Stmt>),
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

    fn block(&mut self, block_terminators: &[TokenKind]) -> ParseResult<Stmt> {
        let mut stmts = Vec::new();
        while !self.match_tokens(block_terminators) {
            stmts.push(self.declaration()?);
        }
        Ok(Stmt::Block(stmts))
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
                    KeywordKind::If => return self.if_stmt(),
                    KeywordKind::Repeat => return self.repeat(),
                    KeywordKind::While => return self.while_stmt(),
                    KeywordKind::For => return self.for_stmt(),
                    _ => ()
                },
                _ => ()
            }
            None => return self.error(
                String::from("expected statement"),
                None,
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

    fn if_stmt(&mut self) -> ParseResult<Stmt> {
        self.tokens.next();
        let condition = self.expr()?;

        /*
        todo: allow alternate IF statement syntax:
        IF condition
            THEN
                <stmts>
        ENDIF
         */

        self.consume(
            TokenKind::Keyword(KeywordKind::Then),
            String::from("expected keyword `THEN` after condition."),
        )?;
        self.consume(
            TokenKind::NewLine,
            String::from("expected new line after keyword, `THEN`."),
        )?;

        let then_branch = Box::new(self.block(&[
            TokenKind::Keyword(KeywordKind::EndIf),
            TokenKind::Keyword(KeywordKind::Else),
        ])?);

        let mut else_branch = None;

        if self.match_tokens(&[TokenKind::Keyword(KeywordKind::Else)]) {
            self.tokens.next();
            self.consume(
                TokenKind::NewLine,
                String::from("expected new line after keyword, `ELSE`."),
            )?;
            else_branch = Some(Box::new(self.block(&[
                TokenKind::Keyword(KeywordKind::EndIf),
            ])?));
        }

        self.consume(
            TokenKind::Keyword(KeywordKind::EndIf),
            String::from("expected `ENDIF` after `IF` statement."),
        )?;

        self.consume(
            TokenKind::NewLine,
            String::from("expected new line after keyword, `ENDIF`."),
        )?;

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch
        })
    }

    fn repeat(&mut self) -> ParseResult<Stmt> {
        self.tokens.next();
        
        self.consume(
            TokenKind::NewLine,
            String::from("expected new line after keyword, `REPEAT`."),
        )?;

        let body = Box::new(self.block(&[
            TokenKind::Keyword(KeywordKind::Until)
        ])?);

        self.consume(
            TokenKind::Keyword(KeywordKind::Until),
            String::from("expected keyword, `UNTIL`, after post-condition loop body."),
        )?;

        let condition = self.expr()?;

        self.consume(
            TokenKind::NewLine,
            String::from("expected new line after REPEAT loop condition."),
        )?;
        
        Ok(Stmt::Repeat { body, until: condition })
    }

    fn while_stmt(&mut self) -> ParseResult<Stmt> {
        self.tokens.next();
        let condition = self.expr()?;

        self.consume(
            TokenKind::NewLine,
            String::from("expected new line after WHILE loop condition."),
        )?;

        let body = Box::new(self.block(&[
            TokenKind::Keyword(KeywordKind::EndWhile)
        ])?);

        self.consume(
            TokenKind::Keyword(KeywordKind::EndWhile),
            String::from("expected keyword, `ENDWHILE`, after pre-condition loop body."),
        )?;

        self.consume(
            TokenKind::NewLine,
            String::from("expected new line after keyword, `ENDWHILE`."),
        )?;
        
        Ok(Stmt::While { body, condition })
    }

    fn for_stmt(&mut self) -> ParseResult<Stmt> {
        self.tokens.next();
        let initializer = self.expr()?;

        self.consume(
            TokenKind::Keyword(KeywordKind::To),
            String::from("expected keyword, `TO`, after initializer expression."),
        )?;

        // todo: ensure to expr is an assignment
        let to = self.expr()?;

        let step = match self.match_tokens(&[TokenKind::Keyword(KeywordKind::Step)]) {
            true => {
                self.tokens.next();
                Some(self.expr()?)
            },
            false => None
        };

        self.consume(
            TokenKind::NewLine,
            String::from("expected new line after `FOR` loop header."),
        )?;

        let body = Box::new(self.block(&[
            TokenKind::Keyword(KeywordKind::Next)
        ])?);

        self.consume(
            TokenKind::Keyword(KeywordKind::Next),
            String::from("expected keyword, `NEXT`, after count-controlled loop body."),
        )?;

        // todo: ensure this is a variable expression
        let counter = self.expr()?;

        self.consume(
            TokenKind::NewLine,
            String::from("expected new line after identifier."),
        )?;

        Ok(Stmt::For {
            initializer,
            to,
            step,
            body,
            counter
        })

    }
}