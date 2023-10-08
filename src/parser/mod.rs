mod error;
mod expr;
mod stmt;
mod type_name;

use std::iter::Peekable;
use crate::lexer::token::{Token, TokenKind};
use crate::parser::error::ParseResult;

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

    fn match_tokens(&mut self, kinds: &[TokenKind]) -> bool {
        match self.tokens.peek() {
            Some(token) => {
                let mut matched = false;
                for kind in kinds.iter() {
                    if &token.kind == kind {
                        matched = true;
                    }
                }
                matched
            }
            None => false
        }
    }
}