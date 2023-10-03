use crate::lexer::token::Token;
use crate::parser::Parser;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub struct ParseError {
    msg: String,
    token: Option<Token>,
}

impl ParseError {
    pub fn new(msg: String, token: Option<Token>) -> Self {
        Self {
            msg,
            token
        }
    }
}

impl<I> Parser<I>
where I: Iterator<Item = Token>
{
    #[inline]
    pub fn error<T>(&mut self, msg: String, token: Option<Token>) -> ParseResult<T> {
        self.had_error = true;
        Err(ParseError::new(msg, token))
    }
}