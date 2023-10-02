mod error;
mod expr;
mod stmt;

use std::iter::Peekable;
use crate::lexer::token::Token;



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