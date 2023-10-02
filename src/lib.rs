mod lexer;
mod parser;
#[allow(dead_code)] mod vm;

use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn interpret(src: &str) {
    let tokens = Lexer::new(src);
    let mut parser = Parser::new(tokens.peekable());
    println!("{:?}", parser.expr());
}