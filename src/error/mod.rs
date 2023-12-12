use crate::lexer::token::Token;

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug)]
pub struct ParseError {
    pub msg: &'static str,
    pub token: Option<Token>,
}

pub fn print_parse_errors(src: &str, errors: Vec<ParseError>) {
    errors.into_iter().for_each(|error| {
        print_error(src, error);
        println!();
        println!();
        println!();
    });
}

fn print_error(src: &str, error: ParseError) {
    match &error.token {
        Some(token) => {
            let line = src.lines().nth(token.line).unwrap();
            let line_number = token.line + 1;
            let line_number_len = line_number.to_string().len();
            println!("{}:{}", line_number, token.col);
            println!("{} |", " ".repeat(line_number_len));
            println!("{} |\t{}", line_number, line);
            println!(
                "{} | \t{}{}",
                " ".repeat(line_number_len),
                " ".repeat(token.col - 1),
                "^".repeat(token.len)
            );
            println!("error: {}", error.msg);
            println!("got `{:?}`", token)
        }
        None => println!("error: {}", error.msg),
    }
}

impl ParseError {
    pub fn new(msg: &'static str, token: Option<Token>) -> Self {
        Self { msg, token }
    }
}