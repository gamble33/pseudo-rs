use crate::lexer::token::Token;

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