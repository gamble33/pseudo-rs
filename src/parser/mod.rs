mod error;
mod expr;
mod stmt;

use std::iter::Peekable;
use crate::lexer::token::{Token, TokenKind};
use crate::parser::error::{ParseResult, ParseError};

pub struct Parser<I>
    where I: Iterator<Item=Token>
{
    tokens: Peekable<I>,
}

impl<I> Parser<I>
    where I: Iterator<Item=Token>
{
    pub fn new(tokens: Peekable<I>) -> Self {
        Self {
            tokens,
        }
    }

    fn consume(&mut self, kind: TokenKind, msg: String) -> ParseResult<()> {
        match self.tokens.next() {
            Some(t) => {
                if t.kind != kind {
                    return Err(ParseError::new(msg, Some(t)))
                }
                return Ok(())
            }
            None => Err(ParseError::new(msg, None))
        }
    }
}

// todo: fix macro so it allows TokenKind::Equal (instead of just Equal)
#[macro_export]
macro_rules! match_tokens {
    ( $self:ident, $( $kind:ident ),* ) => {
        match $self.tokens.peek() {
            Some(t) => match t.kind {
                $(
                    TokenKind::$kind => true,
                )*
                _ => false
            },
            _ => false
        }
    };
}