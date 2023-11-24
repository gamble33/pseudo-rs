mod error;
pub mod expr;
pub mod stmt;
#[allow(dead_code)]
pub mod type_name;

use std::iter::Peekable;
use crate::lexer::token::{KeywordKind, Token, TokenKind};
use crate::parser::error::{ParseError, ParseResult};
use crate::ir::ast::Decl;

pub struct Parser<I>
    where I: Iterator<Item=Token>
{
    tokens: Peekable<I>,
}

impl<I> Parser<I>
    where I: Iterator<Item=Token>
{
    pub fn new(tokens: Peekable<I>) -> Self {
        Self { tokens }
    }

    pub fn program(&mut self) -> Result<Vec<Decl>, Vec<ParseError>> {
        let mut declarations = Vec::new();
        let mut errors = Vec::new();
        let mut had_error = false;

        while self.tokens.peek().is_some() {
            match self.decl() {
                Ok(decl) => declarations.push(decl),
                Err(error) => {
                    had_error = true;
                    errors.push(error);
                    self.synchronize();
                }
            }
        }

        match had_error {
            true => Err(errors),
            false => Ok(declarations)
        }
    }

    fn synchronize(&mut self) {
        while !self.match_tokens(&[
            TokenKind::Keyword(KeywordKind::Procedure),
            TokenKind::Keyword(KeywordKind::Function),
        ]) && self.tokens.peek().is_some() {
            self.tokens.next();
        }
    }

    fn consume(&mut self, kind: TokenKind, msg: String) -> ParseResult<Token> {
        match self.tokens.next() {
            Some(token) => {
                if token.kind != kind {
                    return self.error(msg, Some(token));
                }
                return Ok(token);
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

pub fn print_parse_errors(errors: Vec<ParseError>) {
    errors.iter().for_each(|error| {
        println!("error: {}", error.msg);
        match &error.token {
            Some(token) => println!("got `{:?}`", token),
            None => (),
        }
    });
}
