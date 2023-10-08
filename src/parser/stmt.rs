use crate::lexer::token::{KeywordKind, Token, TokenKind};
use crate::parser::error::ParseResult;
use crate::parser::expr::{Expr, LiteralKind};
use crate::parser::Parser;
use crate::parser::type_name::TypeName;

#[derive(Debug)]
pub enum Stmt {
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },

    Repeat {
        body: Box<Stmt>,
        until: Expr,
    },

    While {
        body: Box<Stmt>,
        condition: Expr,
    },

    Procedure {
        name: String,
        params: Vec<Param>,
        body: Box<Stmt>,
    },

    Expr(Expr),
    Output(Expr),
    Input(Expr),
    Block(Vec<Stmt>),
}

#[derive(Debug)]
pub struct Param {
    name: String,
    type_name: TypeName,
    passing_mode: Option<PassingMode>,
}

#[derive(Debug)]
pub enum PassingMode {
    ByVal,
    ByRef
}

impl<I> Parser<I>
    where I: Iterator<Item=Token>
{
    pub fn decl(&mut self) -> ParseResult<Stmt> {
        match self.tokens.next() {
            Some(token) => match &token.kind {
                TokenKind::Keyword(keyword) => match keyword {
                    KeywordKind::Procedure => self.procedure(),
                    _ => self.error(
                        String::from("expected declaration."),
                        Some(token),
                    )
                },
                _ => self.error(
                    String::from("expected declaration."),
                    Some(token),
                )
            }
            None => self.error(
                String::from("expected declaration."),
                None,
            )
        }
    }

    fn block(&mut self, block_terminators: &[TokenKind]) -> ParseResult<Stmt> {
        let mut stmts = Vec::new();
        while !self.match_tokens(block_terminators) {
            stmts.push(self.stmt()?);
        }
        Ok(Stmt::Block(stmts))
    }


    fn stmt(&mut self) -> ParseResult<Stmt> {
        match self.tokens.peek() {
            Some(t) => match &t.kind {
                TokenKind::Keyword(keyword) => match keyword {
                    KeywordKind::Output => self.output(),
                    KeywordKind::Input => self.input(),
                    KeywordKind::If => self.if_stmt(),
                    KeywordKind::Repeat => self.repeat(),
                    KeywordKind::While => self.while_stmt(),
                    KeywordKind::For => self.for_stmt(),
                    KeywordKind::Declare => self.var_decl(),
                    _ => self.expr_stmt()

                },
                _ => self.expr_stmt()
            }
            None => return self.error(
                String::from("expected statement"),
                None,
            )
        }
    }

    fn param(&mut self) -> ParseResult<Param> {
        let passing_mode = match self.tokens.peek() {
            Some(token) => match &token.kind {
                TokenKind::Keyword(keyword) => match keyword {
                    KeywordKind::ByRef => {
                        self.tokens.next();
                        Some(PassingMode::ByRef)
                    },
                    KeywordKind::ByVal => {
                        self.tokens.next();
                        Some(PassingMode::ByVal)
                    },
                    _ => None,
                }
                _ => None,
            },
            None => None,
        };

        let name = match self.tokens.next() {
            Some(token) => match token.kind {
                TokenKind::Identifier(name) => name,
                _ => return self.error(
                    String::from("expected identifier for parameter name."),
                    Some(token),
                )
            }
            None => return self.error(
                String::from("expected identifier for parameter name."),
                None,
            )
        };

        self.consume(
            TokenKind::Colon,
            String::from("expected `:` after parameter name.")
        )?;

        let type_name = self.type_name()?;

        Ok(Param {
            passing_mode,
            name,
            type_name
        })
    }

    fn procedure(&mut self) -> ParseResult<Stmt> {
        let name = match self.tokens.next() {
            Some(token) => match token.kind {
                TokenKind::Identifier(name) => name,
                _ => return self.error(
                    String::from("expected identifier for PROCEDURE name."),
                    Some(token),
                )
            }
            None => return self.error(
                String::from("expected identifier for PROCEDURE name."),
                None,
            )
        };
        let mut params = Vec::new();
        if self.match_tokens(&[TokenKind::OpenParen]) {
            self.tokens.next();

            loop {
                params.push(self.param()?);

                if !self.match_tokens(&[TokenKind::Comma]) {
                    break;
                }
                self.tokens.next();
            }

            self.consume(
                TokenKind::CloseParen,
                String::from("expected `)` after parameters.")
            )?;
        }

        self.consume(
            TokenKind::NewLine,
            String::from("expected new line after PROCEDURE header."),
        )?;

        let body = Box::new(self.block(&[
            TokenKind::Keyword(KeywordKind::EndProcedure),
        ])?);

        self.tokens.next();

        Ok(Stmt::Procedure {
            name,
            params,
            body
        })
    }

    fn var_decl(&mut self) -> ParseResult<Stmt> {
        self.tokens.next();
        todo!()
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

    fn input(&mut self) -> ParseResult<Stmt> {
        self.tokens.next();
        let holder = self.expr()?;
        self.consume(
            TokenKind::NewLine,
            String::from("expected new line after expression."),
        )?;
        Ok(Stmt::Input(holder))
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
            else_branch,
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
            }
            false => None
        };

        self.consume(
            TokenKind::NewLine,
            String::from("expected new line after `FOR` loop header."),
        )?;

        let body = self.block(&[
            TokenKind::Keyword(KeywordKind::Next)
        ])?;

        self.consume(
            TokenKind::Keyword(KeywordKind::Next),
            String::from("expected keyword, `NEXT`, after count-controlled loop body."),
        )?;

        let counter = self.expr()?;

        match counter {
            Expr::Variable(_) => (),
            _ => self.error(
                String::from("FOR loop must specify variable to increment."),
                None, // todo: figure out how to insert token here.
            )?,
        }

        self.consume(
            TokenKind::NewLine,
            String::from("expected new line after identifier."),
        )?;

        // de-sugaring FOR loops into WHILE loops
        Ok(Stmt::Block(vec![
            Stmt::Expr(initializer),
            Stmt::While {
                body: Box::new(Stmt::Block(vec![
                    body,
                    Stmt::Expr(Expr::Assignment {
                        target: Box::new(counter.clone()),
                        value: Box::new(Expr::Binary {
                            lhs: Box::new(counter.clone()),
                            op: Token::new(TokenKind::Plus),
                            rhs: Box::new(match step {
                                Some(s) => s,
                                None => Expr::Literal(LiteralKind::Integer(1))
                            }),
                        }),
                    }),
                ])),
                condition: Expr::Binary {
                    lhs: Box::new(counter),
                    op: Token::new(TokenKind::NotEqual),
                    rhs: Box::new(to),
                },
            },
        ]))
    }
}