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
    had_error: bool,
}

impl<I> Parser<I>
    where I: Iterator<Item=Token>
{
    pub fn new(tokens: Peekable<I>) -> Self {
        Self {
            tokens,
            had_error: false,
        }
    }

    fn consume(&mut self, kind: TokenKind, msg: String) -> ParseResult<()> {
        match self.tokens.next() {
            Some(t) => {
                if t.kind != kind {
                    return self.error(msg, Some(t))
                }
                return Ok(())
            }
            None => self.error(msg, None)
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