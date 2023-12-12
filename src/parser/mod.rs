pub mod error;
pub mod expr;
pub mod stmt;
#[allow(dead_code)]
pub mod type_name;

use crate::ir::ast::Decl;
use crate::lexer::token::{KeywordKind, Token, TokenKind};
use crate::parser::error::{ParseError, ParseResult};
use std::iter::Peekable;

pub struct Parser<I>
where
    I: Iterator<Item = Token>,
{
    tokens: Peekable<I>,
    had_error: bool,
    errors: Vec<ParseError>,
}

pub fn program<I>(tokens: Peekable<I>) -> Result<Vec<Decl>, Vec<ParseError>>
where
    I: Iterator<Item = Token>,
{
    let mut parser = Parser::new(tokens);
    let mut declarations = Vec::new();

    while parser.tokens.peek().is_some() {
        match parser.decl() {
            Ok(decl) => declarations.push(decl),
            Err(error) => {
                parser.had_error = true;
                parser.errors.push(error);
                parser.synchronize_decl();
            }
        }
    }

    match parser.had_error {
        true => Err(parser.errors),
        false => Ok(declarations),
    }
}

impl<I> Parser<I>
where
    I: Iterator<Item = Token>,
{
    fn new(tokens: Peekable<I>) -> Self {
        Self {
            tokens,
            had_error: false,
            errors: Vec::new(),
        }
    }

    fn synchronize_decl(&mut self) {
        while !self.match_tokens(&[
            TokenKind::Keyword(KeywordKind::Function),
            TokenKind::Keyword(KeywordKind::Procedure),
        ]) && self.tokens.peek().is_some()
        {
            self.tokens.next();
        }
    }

    fn synchronize_stmt(&mut self) {
        while !self.match_tokens(&[TokenKind::NewLine]) && self.tokens.peek().is_some() {
            self.tokens.next();
        }
        self.tokens.next();
    }

    fn consume(&mut self, kind: TokenKind, msg: String) -> ParseResult<Token> {
        match self.tokens.next() {
            Some(token) => {
                if token.kind != kind {
                    return self.error(msg, Some(token));
                }
                return Ok(token);
            }
            None => self.error(msg, None),
        }
    }

    fn match_tokens(&mut self, kinds: &[TokenKind]) -> bool {
        match self.tokens.peek() {
            Some(token) => {
                let mut matched = false;
                // todo: use slice.contains() method instead
                for kind in kinds.iter() {
                    if &token.kind == kind {
                        matched = true;
                    }
                }
                matched
            }
            None => false,
        }
    }
}
