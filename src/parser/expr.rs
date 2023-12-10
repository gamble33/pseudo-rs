use crate::ir::ast::{ExprKind, LiteralKind};
use crate::lexer::token::{KeywordKind::*, Token, TokenKind::*, TokenLiteralKind};
use crate::parser::error::ParseResult;
use crate::parser::Parser;

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    pub fn expr(&mut self) -> ParseResult<ExprKind> {
        self.assignment()
    }

    fn assignment(&mut self) -> ParseResult<ExprKind> {
        let expr = self.or()?;

        if self.match_tokens(&[LeftArrow]) {
            self.tokens.next();
            // Note: This allows chained assignment syntax `a <- b <- c`.
            // Might need to change this...
            return Ok(ExprKind::Assignment {
                target: match expr {
                    ExprKind::Variable(name) => name,
                    _ => self.error(String::from("invalid assignment target"), None)?,
                },
                value: Box::new(self.expr()?),
            });
        }

        Ok(expr)
    }

    fn or(&mut self) -> ParseResult<ExprKind> {
        let mut expr = self.and()?;

        while self.match_tokens(&[Keyword(Or)]) {
            let op = self.tokens.next().unwrap();
            let rhs = self.and()?;
            expr = ExprKind::Logical {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            }
        }

        Ok(expr)
    }

    fn and(&mut self) -> ParseResult<ExprKind> {
        let mut expr = self.equality()?;

        while self.match_tokens(&[Keyword(And)]) {
            let op = self.tokens.next().unwrap();
            let rhs = self.equality()?;
            expr = ExprKind::Logical {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            }
        }

        Ok(expr)
    }

    // todo: right a macro for left-associative binary operations.

    fn equality(&mut self) -> ParseResult<ExprKind> {
        let mut expr = self.comparison()?;

        while self.match_tokens(&[Equal, NotEqual]) {
            let op = self.tokens.next().unwrap();
            let rhs = self.comparison()?;
            expr = ExprKind::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> ParseResult<ExprKind> {
        let mut expr = self.term()?;

        while self.match_tokens(&[
            Greater,
            GreaterEqual,
            Less,
            LessEqual,
        ]) {
            let op = self.tokens.next().unwrap();
            let rhs = self.term()?;
            expr = ExprKind::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> ParseResult<ExprKind> {
        let mut expr = self.factor()?;

        while self.match_tokens(&[Plus, Minus, Ampersand]) {
            let op = self.tokens.next().unwrap();
            let rhs = self.factor()?;
            expr = ExprKind::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> ParseResult<ExprKind> {
        let mut expr = self.unary()?;

        while self.match_tokens(&[
            Star,
            Slash,
            Keyword(Div),
            Keyword(Mod),
        ]) {
            let op = self.tokens.next().unwrap();
            let rhs = self.unary()?;
            expr = ExprKind::Binary {
                lhs: Box::new(expr),
                op,
                rhs: Box::new(rhs),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> ParseResult<ExprKind> {
        // todo: exhaust list of unary operators
        if self.match_tokens(&[Minus, Keyword(Not)]) {
            return Ok(ExprKind::Unary {
                op: self.tokens.next().unwrap(),
                expr: Box::new(self.unary()?),
            });
        }

        self.call_expr()
    }

    fn call_expr(&mut self) -> ParseResult<ExprKind> {
        let mut expr = self.primary()?;

        if self.match_tokens(&[OpenParen]) {
            self.tokens.next();
            let mut args = Vec::new();
            loop {
                args.push(self.expr()?);
                if !self.match_tokens(&[Comma]) {
                    break;
                }
                self.tokens.next();
            }
            self.consume(
                CloseParen,
                String::from("expected `)` after arguments"),
            )?;
            expr = ExprKind::Call {
                callee: Box::new(expr),
                args,
            }
        }
        Ok(expr)
    }

    fn primary(&mut self) -> ParseResult<ExprKind> {
        Ok(match self.tokens.next() {
            Some(t) => match &t.kind {
                Literal(literal) => match literal {
                    TokenLiteralKind::Integer(i) => ExprKind::Literal(LiteralKind::Integer(*i)),
                    TokenLiteralKind::Real(f) => ExprKind::Literal(LiteralKind::Real(*f)),
                    TokenLiteralKind::Character(ch) => {
                        ExprKind::Literal(LiteralKind::Character(*ch))
                    }
                    TokenLiteralKind::Str(string) => {
                        ExprKind::Literal(LiteralKind::String(string.to_owned()))
                    }
                },
                Keyword(keyword) => match keyword {
                    True => ExprKind::Literal(LiteralKind::Boolean(true)),
                    False => ExprKind::Literal(LiteralKind::Boolean(false)),
                    _ => {
                        return self.error(
                            String::from("expected literal, identifier or grouping (not keyword)"),
                            Some(t),
                        )
                    }
                },
                Identifier(name) => ExprKind::Variable(name.to_owned()),
                OpenParen => {
                    let expr = self.expr()?;
                    self.consume(
                        CloseParen,
                        String::from("expected closing `)` after grouping expression"),
                    )?;
                    expr
                }
                _ => {
                    return self.error(
                        String::from("expected literal, identifier or grouping"),
                        Some(t),
                    )
                }
            },
            None => {
                return self.error(
                    String::from("expected literal, identifier or grouping"),
                    None,
                )
            }
        })
    }
}
