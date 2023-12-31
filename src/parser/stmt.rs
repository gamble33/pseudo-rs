use crate::ir::ast::{Decl, ExprKind, LiteralKind, Param, PassingMode, Stmt};
use crate::lexer::token::{KeywordKind, Token, TokenKind};
use crate::error::ParseResult;
use crate::parser::Parser;

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    pub fn decl(&mut self) -> ParseResult<Decl> {
        let token = self.tokens.next();
        let decl = match token {
            Some(tok) => match tok.kind {
                TokenKind::Keyword(ref keyword) => match keyword {
                    KeywordKind::Procedure => self.procedure(tok),
                    KeywordKind::Function => self.function(tok),
                    _ => self.error("expected declaration.", Some(tok)),
                },
                _ => self.error("expected declaration.", Some(tok)),
            },
            None => self.error("expected declaration.", None),
        }?;

        match self.tokens.next() {
            Some(token) => match token.kind {
                TokenKind::NewLine => Ok(decl),
                _ => self.error(
                    "expected new line after declaration.",
                    Some(token),
                ),
            },
            None => Ok(decl),
        }
    }

    fn block(&mut self, block_terminators: &[TokenKind], block_decl: Token) -> ParseResult<Stmt> {
        let mut stmts = Vec::new();
        let mut block_terminated = false;
        loop {
            if self.match_tokens(block_terminators) {
                block_terminated = true;
                break;
            } else if self.tokens.peek().is_none() {
                break;
            }
            let stmt = match self.stmt() {
                Ok(stmt) => stmt,
                Err(err) => {
                    self.had_error = true;
                    self.errors.push(err);
                    self.synchronize_stmt();
                    continue;
                }
            };
            stmts.push(stmt);
        }
        match block_terminated {
            true => Ok(Stmt::Block(stmts)),
            false => self.error("Block unterminated", Some(block_decl))?,
        }
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
                    KeywordKind::Call => self.call(),
                    KeywordKind::Return => self.return_stmt(),
                    _ => self.expr_stmt(),
                },
                _ => self.expr_stmt(),
            },
            None => return self.error("expected statement", None),
        }
    }

    fn param(&mut self) -> ParseResult<Param> {
        let passing_mode = match self.tokens.peek() {
            Some(token) => match &token.kind {
                TokenKind::Keyword(keyword) => match keyword {
                    KeywordKind::ByRef => {
                        self.tokens.next();
                        Some(PassingMode::ByRef)
                    }
                    KeywordKind::ByVal => {
                        self.tokens.next();
                        Some(PassingMode::ByVal)
                    }
                    _ => None,
                },
                _ => None,
            },
            None => None,
        };

        let name = match self.tokens.next() {
            Some(token) => match token.kind {
                TokenKind::Identifier(name) => name,
                _ => {
                    return self.error(
                        "expected identifier for parameter name.",
                        Some(token),
                    )
                }
            },
            None => {
                return self.error(
                    "expected identifier for parameter name.",
                    None,
                )
            }
        };

        self.consume(
            TokenKind::Colon,
            "expected `:` after parameter name.",
        )?;

        let type_name = self.type_name()?;

        Ok(Param {
            passing_mode,
            name,
            type_name,
        })
    }

    fn params(&mut self) -> ParseResult<Vec<Param>> {
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
                "expected `)` after parameters.",
            )?;
        }
        Ok(params)
    }

    fn procedure(&mut self, procedure_keyword: Token) -> ParseResult<Decl> {
        let name = match self.tokens.next() {
            Some(token) => match token.kind {
                TokenKind::Identifier(name) => name,
                _ => {
                    return self.error(
                        "expected identifier for PROCEDURE name.",
                        Some(token),
                    )
                }
            },
            None => {
                return self.error(
                    "expected identifier for PROCEDURE name.",
                    None,
                )
            }
        };

        let params = self.params()?;

        self.consume(
            TokenKind::NewLine,
            "expected new line after PROCEDURE header.",
        )?;

        let body = self.block(&[TokenKind::Keyword(KeywordKind::EndProcedure)], procedure_keyword)?;

        self.tokens.next();

        Ok(Decl::Procedure { name, params, body })
    }

    fn function(&mut self, function_keyword: Token) -> ParseResult<Decl> {
        let name = match self.tokens.next() {
            Some(token) => match token.kind {
                TokenKind::Identifier(name) => name,
                _ => {
                    return self.error(
                        "expected identifier for FUNCTION name.",
                        Some(token),
                    )
                }
            },
            None => {
                return self.error("expected identifier for FUNCTION name.", None)
            }
        };

        let params = self.params()?;

        self.consume(
            TokenKind::Keyword(KeywordKind::Returns),
            "expected keyword `RETURNS` after FUNCTION declaration",
        )?;

        let return_type_name = self.type_name()?;

        self.consume(
            TokenKind::NewLine,
            "expected new line after FUNCTION return type.",
        )?;

        let body = self.block(&[TokenKind::Keyword(KeywordKind::EndFunction)], function_keyword)?;

        self.tokens.next();

        Ok(Decl::Function {
            name,
            params,
            body,
            return_type_name,
        })
    }

    fn call(&mut self) -> ParseResult<Stmt> {
        self.tokens.next();

        let name = match self.tokens.next() {
            Some(token) => match token.kind {
                TokenKind::Identifier(name) => name,
                _ => {
                    return self.error(
                            "expected identifier for PROCEDURE name after keyword, `CALL`.",
                        Some(token),
                    )
                }
            },
            None => {
                return self.error(
                    "expected identifier for PROCEDURE name after keyword, `CALL`.",
                    None,
                )
            }
        };

        let mut args = Vec::new();
        if self.match_tokens(&[TokenKind::OpenParen]) {
            self.tokens.next();

            loop {
                args.push(self.expr()?);

                if !self.match_tokens(&[TokenKind::Comma]) {
                    break;
                }
                self.tokens.next();
            }

            self.consume(
                TokenKind::CloseParen,
                "expected `)` after arguments.",
            )?;
        }

        self.consume(
            TokenKind::NewLine,
            "expected new line after procedure call.",
        )?;

        Ok(Stmt::Call { name, args })
    }

    fn return_stmt(&mut self) -> ParseResult<Stmt> {
        self.tokens.next();
        let expr = self.expr()?;
        self.consume(
            TokenKind::NewLine,
            "expected new line after expression.",
        )?;
        Ok(Stmt::Return(expr))
    }

    fn var_decl(&mut self) -> ParseResult<Stmt> {
        self.tokens.next();

        let name = match self.tokens.next() {
            Some(token) => match token.kind {
                TokenKind::Identifier(name) => name,
                _ => {
                    return self.error(
                        "expected identifier for variable name.",
                        Some(token),
                    )
                }
            },
            None => {
                return self.error("expected identifier for variable name.", None)
            }
        };

        self.consume(
            TokenKind::Colon,
            "expected `:` after variable name.",
        )?;

        let type_name = self.type_name()?;

        self.consume(
            TokenKind::NewLine,
            "expected new line after variable declaration.",
        )?;

        Ok(Stmt::VarDecl { name, type_name })
    }

    fn expr_stmt(&mut self) -> ParseResult<Stmt> {
        let expr_stmt = Stmt::Expr(self.expr()?);
        self.consume(
            TokenKind::NewLine,
            "expected new line after expression.",
        )?;
        Ok(expr_stmt)
    }

    fn output(&mut self) -> ParseResult<Stmt> {
        self.tokens.next();

        let mut exprs = Vec::new();
        loop {
            exprs.push(self.expr()?);
            if !self.match_tokens(&[TokenKind::Comma]) {
                break;
            }
            self.tokens.next();
        }
        self.consume(
            TokenKind::NewLine,
            "expected new line after expression.",
        )?;
        Ok(Stmt::Output(exprs))
    }

    fn input(&mut self) -> ParseResult<Stmt> {
        self.tokens.next();
        let holder = match self.expr()? {
            ExprKind::Variable(name) => name,

            // todo: Add token previous
            _ => self.error("Cannot store input in that", None)?,
        };
        self.consume(
            TokenKind::NewLine,
            "expected new line after expression.",
        )?;
        Ok(Stmt::Input(holder))
    }

    fn if_stmt(&mut self) -> ParseResult<Stmt> {
        let if_keyword = self.tokens.next().unwrap();
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
            "expected keyword `THEN` after condition.",
        )?;
        self.consume(
            TokenKind::NewLine,
            "expected new line after keyword, `THEN`.",
        )?;

        let then_branch = Box::new(self.block(&[
            TokenKind::Keyword(KeywordKind::EndIf),
            TokenKind::Keyword(KeywordKind::Else),
        ], if_keyword.clone())?);

        let mut else_branch = None;

        if self.match_tokens(&[TokenKind::Keyword(KeywordKind::Else)]) {
            self.tokens.next();
            self.consume(
                TokenKind::NewLine,
                "expected new line after keyword, `ELSE`.",
            )?;
            else_branch = Some(Box::new(
                self.block(&[TokenKind::Keyword(KeywordKind::EndIf)], if_keyword)?,
            ));
        }

        self.consume(
            TokenKind::Keyword(KeywordKind::EndIf),
            "expected `ENDIF` after `IF` statement.",
        )?;

        self.consume(
            TokenKind::NewLine,
            "expected new line after keyword, `ENDIF`.",
        )?;

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn repeat(&mut self) -> ParseResult<Stmt> {
        let repeat_keyword = self.tokens.next().unwrap();

        self.consume(
            TokenKind::NewLine,
            "expected new line after keyword, `REPEAT`.",
        )?;

        let body = Box::new(self.block(&[TokenKind::Keyword(KeywordKind::Until)], repeat_keyword)?);

        self.consume(
            TokenKind::Keyword(KeywordKind::Until),
            "expected keyword, `UNTIL`, after post-condition loop body.",
        )?;

        let condition = self.expr()?;

        self.consume(
            TokenKind::NewLine,
            "expected new line after REPEAT loop condition.",
        )?;

        Ok(Stmt::Repeat {
            body,
            until: condition,
        })
    }

    fn while_stmt(&mut self) -> ParseResult<Stmt> {
        let while_keyword = self.tokens.next().unwrap();
        let condition = self.expr()?;

        self.consume(
            TokenKind::NewLine,
            "expected new line after WHILE loop condition.",
        )?;

        let body = Box::new(self.block(&[TokenKind::Keyword(KeywordKind::EndWhile)], while_keyword)?);

        self.consume(
            TokenKind::Keyword(KeywordKind::EndWhile),
            "expected keyword, `ENDWHILE`, after pre-condition loop body.",
        )?;

        self.consume(
            TokenKind::NewLine,
            "expected new line after keyword, `ENDWHILE`.",
        )?;

        Ok(Stmt::While { body, condition })
    }

    fn for_stmt(&mut self) -> ParseResult<Stmt> {
        let for_keyword = self.tokens.next().unwrap();
        let initializer = self.expr()?;

        self.consume(
            TokenKind::Keyword(KeywordKind::To),
            "expected keyword, `TO`, after initializer expression.",
        )?;

        // todo: ensure to expr is an assignment
        let to = self.expr()?;

        let step = match self.match_tokens(&[TokenKind::Keyword(KeywordKind::Step)]) {
            true => {
                self.tokens.next();
                Some(self.expr()?)
            }
            false => None,
        };

        self.consume(
            TokenKind::NewLine,
            "expected new line after `FOR` loop header.",
        )?;

        let body = self.block(&[TokenKind::Keyword(KeywordKind::Next)], for_keyword)?;

        self.consume(
            TokenKind::Keyword(KeywordKind::Next),
            "expected keyword, `NEXT`, after count-controlled loop body.",
        )?;

        let counter = self.expr()?;

        match counter {
            ExprKind::Variable(_) => (),
            _ => self.error(
                "FOR loop must specify variable to increment.",
                None, // todo: figure out how to insert token here.
            )?,
        }

        self.consume(
            TokenKind::NewLine,
            "expected new line after identifier.",
        )?;

        // todo: account for imperfect steps
        // FOR i <- 0 TO 10 STEP 3      results in infinite loop.

        // de-sugaring FOR loops into WHILE loops
        Ok(Stmt::Block(vec![
            Stmt::Expr(initializer),
            Stmt::While {
                body: Box::new(Stmt::Block(vec![
                    body,
                    Stmt::Expr(ExprKind::Assignment {
                        target: match counter.clone() {
                            ExprKind::Variable(name) => name,
                            _ => self.error(
                                "invalid FOR loop increment variable.",
                                None,
                            )?,
                        },
                        value: Box::new(ExprKind::Binary {
                            lhs: Box::new(counter.clone()),
                            op: Token::from(TokenKind::Plus),
                            rhs: Box::new(match step {
                                Some(s) => s,
                                None => ExprKind::Literal(LiteralKind::Integer(1)),
                            }),
                        }),
                    }),
                ])),
                condition: ExprKind::Binary {
                    lhs: Box::new(counter),
                    op: Token::from(TokenKind::NotEqual),
                    rhs: Box::new(to),
                },
            },
        ]))
    }
}
