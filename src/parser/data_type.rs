use crate ::parser::Parser;
use crate::parser::error::ParseResult;
use crate::lexer::token::{Token, TokenKind, KeywordKind};

pub enum Type {
    Array {

    }
}

pub enum BaseType {
    Integer,
    String,
    Char,
    Boolean,
    Date,
    UserDefined,
}

impl<I> Parser<I> 
where I: Iterator<Item = Token>
{
/*
 *
 * type -> ARRAY [ <range> ( , <range> )* ] OF <base type>
 *         | <base type> ;
 * 
 * <base type> ->   INTEGER
 *                | STRING
 *                | CHAR
 *                | REAL
 *                | BOOLEAN
 *                | DATE
 *                | <identifier>
 * 
 * range -> <int> : <int>
 */

    fn type_name(&mut self) -> ParseResult<Type> {
        match self.tokens.next() {
            Some(token) => match token.kind {
                TokenKind::Keyword(keyword) => match keyword {
                    KeywordKind::Array => return self.array_type_name(),
                    _ => ()
                },
                _ => ()
            }
            None => self.error(String::from("expected type."), None)?
        };
        self.base_type_name()
    }

    fn array_type_name(&mut self) -> ParseResult<Type> {
        todo!()
    }

    fn base_type_name(&mut self) -> ParseResult<Type> {
        todo!()
    }

}