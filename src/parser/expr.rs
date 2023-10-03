use crate::lexer::token::{TokenLiteralKind, Token, TokenKind, KeywordKind};
use crate::match_tokens;
use crate::parser::Parser;
use crate::parser::error::ParseResult;

#[derive(Debug)]
pub enum Expr {
    Binary {
        lhs: Box<Expr>,
        op: Token,
        rhs: Box<Expr>,
    },
    Unary {
        op: Token,
        expr: Box<Expr>,
    },
    Assignment {
        target: Box<Expr>,
        value: Box<Expr>
    },
    Literal(LiteralKind),
    Variable(String),
}

#[derive(Debug)]
pub enum LiteralKind {
    Integer(i32),
    Character(char),
    String(String),
    Boolean(bool),
}

impl<I> Parser<I> where I: Iterator<Item=Token> {
    pub fn expr(&mut self) -> ParseResult<Expr> {
        self.assignment()
    }

    fn assignment(&mut self) -> ParseResult<Expr> {
        let expr = self.equality()?;

        if match_tokens!(self, LeftArrow) {
            // Note: This allows chained assignment syntax `a <- b <- c`.
            // Might need to change this...
            return Ok(Expr::Assignment {
                target: Box::new(expr),
                value: Box::new(self.expr()?)
            })
        }

        Ok(expr)
    }

    // todo: right a macro for left-associative binary operations.

    fn equality(&mut self) -> ParseResult<Expr> {
        let mut expr = self.comparison()?;


        while match_tokens!(self, Equal, NotEqual) {
            let op = self.tokens.next().unwrap();
            let rhs = self.comparison()?;
            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> ParseResult<Expr> {
        let mut expr = self.term()?;

        while match_tokens!(self, Greater, GreaterEqual, Less, LessEqual) {
            let op = self.tokens.next().unwrap();
            let rhs = self.term()?;
            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> ParseResult<Expr> {
        let mut expr = self.factor()?;

        while match_tokens!(self, Plus, Minus) {
            let op = self.tokens.next().unwrap();
            let rhs = self.factor()?;
            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> ParseResult<Expr> {
        let mut expr = self.unary()?;

        while match_tokens!(self, Star, Slash) {
            let op = self.tokens.next().unwrap();
            let rhs = self.unary()?;
            expr = Expr::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> ParseResult<Expr> {
        // todo: exhaust list of unary operators
        if match_tokens!(self, Minus) {
            return Ok(Expr::Unary {
                op: self.tokens.next().unwrap(),
                expr: Box::new(self.unary()?),
            });
        }

        self.primary()
    }

    fn primary(&mut self) -> ParseResult<Expr> {
        use TokenKind::*;

        Ok(match self.tokens.next() {
            Some(t) => match &t.kind {
                Literal(literal) => match literal {
                    TokenLiteralKind::Integer(i) => Expr::Literal(LiteralKind::Integer(*i)),
                    TokenLiteralKind::Character(ch) => Expr::Literal(LiteralKind::Character(*ch)),
                    TokenLiteralKind::String(string) => Expr::Literal(LiteralKind::String(string.to_owned())),
                },
                Keyword(keyword) => match keyword {
                    KeywordKind::True => Expr::Literal(LiteralKind::Boolean(true)),
                    KeywordKind::False => Expr::Literal(LiteralKind::Boolean(false)),
                    _ => return self.error(
                        String::from("expected literal, identifier or grouping (not keyword)"),
                        Some(t),
                    )
                },
                Identifier(name) => Expr::Variable(name.to_owned()),
                OpenParen => {
                    let expr = self.expr()?;
                    self.consume(
                        CloseParen,
                        String::from("expected closing `)` after grouping expression")
                    )?;
                    expr
                }
                _ => return self.error(
                    String::from("expected literal, identifier or grouping"),
                    Some(t),
                )
            }
            None => return self.error(
                String::from("expected literal, identifier or grouping"),
                None,
            )
        })
    }
}