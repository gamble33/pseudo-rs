mod error;
mod expr;
mod stmt;
#[allow(dead_code)] mod type_name;

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

    fn consume(&mut self, kind: TokenKind, msg: String) -> ParseResult<Token> {
        match self.tokens.next() {
            Some(token) => {
                if token.kind != kind {
                    return self.error(msg, Some(token))
                }
                return Ok(token)
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