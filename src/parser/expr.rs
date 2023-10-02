use crate::lexer::token::{TokenLiteralKind, Token, TokenKind, KeywordKind};
use crate::match_tokens;
use crate::parser::{Parser, ParseResult};

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
    Literal(LiteralKind),
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
        self.equality()
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
        Ok(match self.tokens.next() {
            Some(t) => match t.kind {
                TokenKind::Literal(literal) => match literal {
                    TokenLiteralKind::Integer(i) => Expr::Literal(LiteralKind::Integer(i)),
                    TokenLiteralKind::Character(ch) => Expr::Literal(LiteralKind::Character(ch)),
                    TokenLiteralKind::String(string) => Expr::Literal(LiteralKind::String(string)),
                },
                TokenKind::Keyword(keyword) => match keyword {
                    KeywordKind::True => Expr::Literal(LiteralKind::Boolean(true)),
                    KeywordKind::False => Expr::Literal(LiteralKind::Boolean(false)),
                    _ => todo!("expected literal, identifier or grouping (not keyword)")
                },
                TokenKind::OpenParen => {
                    let expr = self.expr();
                    match self.tokens.next() {
                        Some(t) => match t.kind {
                            TokenKind::CloseParen => {
                                return expr;
                            }
                            _ => ()
                        }
                        _ => ()
                    }
                    todo!("Expected closing `)` after grouping expression")
                }
                _ => todo!("expected literal, identifier or grouping")
            }
            None => todo!("expected expression")
        })
    }
}