pub mod token;
mod keyword;
#[cfg(test)] mod tests;

use crate::lexer::token::{Token, TokenKind, TokenLiteralKind};
use std::iter::Peekable;
use std::str::Chars;
use TokenKind::*;

pub struct Lexer<'a> {
    src: Peekable<Chars<'a>>,
    line: usize,
    col: usize,
    current: usize,
    last_token_new_line: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src: src.chars().peekable(),
            line: 0,
            col: 0,
            current: 0,
            last_token_new_line: true,
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let first_char = match self.skip_whitespace_and_get_first_char() {
            Some(ch) => ch,
            None => return None,
        };

        let token_start = self.current - 1;
        let token_col = self.col;
        let token_line = self.line;
        self.last_token_new_line = false;

        let token_kind = match first_char {
            '+' => Plus,
            '-' => Minus,
            '*' => Star,
            '/' => Slash,
            '.' => Dot,
            '(' => OpenParen,
            ')' => CloseParen,
            '[' => OpenSqrBracket,
            ']' => CloseSqrBracket,
            '=' => Equal,
            ',' => Comma,
            ':' => Colon,
            '^' => Caret,
            '&' => Ampersand,

            '<' => match self.src.peek() {
                Some(c) => match c {
                    '>' => {
                        self.advance();
                        NotEqual
                    }
                    '=' => {
                        self.advance();
                        LessEqual
                    }
                    '-' => {
                        self.advance();
                        LeftArrow
                    }
                    _ => Less,
                },
                None => Less,
            },
            '>' => match self.src.peek() {
                Some(c) => match c {
                    '=' => {
                        self.advance();
                        GreaterEqual
                    }
                    _ => Greater,
                },
                None => Greater,
            },

            ch if is_newline(ch) => {
                self.last_token_new_line = true;
                self.new_line();
                NewLine
            }

            '"' => self.string(),
            '\'' => self.character(),
            ch if ch.is_ascii_digit() => self.number(ch),
            ch if ch.is_alphabetic() => self.identifier(ch),

            _ => TokenKind::Error("Invalid token"),
        };

        Some(Token {
            kind: token_kind,
            line: token_line,
            col: token_col,
            len: self.current - token_start,
        })
    }
}

impl Lexer<'_> {
    fn advance(&mut self) -> Option<char> {
        let ch = self.src.next();
        self.current += 1;
        if ch.is_some_and(|ch| !ch.is_control()) {
            self.col += 1;
        }
        ch
    }

    fn new_line(&mut self) {
        self.line += 1;
        self.col = 0;
    }

    fn identifier(&mut self, first_char: char) -> TokenKind {
        let mut value = String::from(first_char);
        self.consume_while(|ch| ch.is_alphabetic() || ch.is_ascii_digit(), &mut value);

        keyword::check_keyword(value)
    }

    fn number(&mut self, first_char: char) -> TokenKind {
        let mut string_value = String::from(first_char);
        self.consume_while(|c| c.is_ascii_digit(), &mut string_value);

        if self.src.peek().is_some_and(|&ch| ch == '.') {
            string_value.push(self.advance().unwrap());
            self.consume_while(|c| c.is_ascii_digit(), &mut string_value);
            match string_value.parse::<f64>() {
                Ok(f) => TokenKind::Literal(TokenLiteralKind::Real(f)),
                Err(_) => TokenKind::Error("REAL literal too large"),
            }
        } else {
            match string_value.parse::<i64>() {
                Ok(i) => TokenKind::Literal(TokenLiteralKind::Integer(i)),
                Err(_) => TokenKind::Error("INTEGER literal too large"),
            }
        }
    }

    fn string(&mut self) -> TokenKind {
        let mut value = String::new();
        self.consume_while(|ch| ch != '"' && !is_newline(ch), &mut value);
        if self.src.peek().is_none()
            || match self.src.peek() {
                Some(&c) if c != '"' => true,
                _ => false,
            }
        {
            todo!("Should not go to parsing stage if this happens");
            TokenKind::Error("Unterminated STRING");
        }
        self.advance();
        TokenKind::Literal(TokenLiteralKind::Str(value))
    }

    fn character(&mut self) -> TokenKind {
        let ch = match self.advance() {
            Some(ch) => ch,
            None => {
                todo!("Should not go to parsing stage if this happens");
                return TokenKind::Error("Expected character after `'`.");
            }
        };
        match self.advance() {
            Some('\'') => (),
            _ => {
                todo!("Should not go to parsing stage if this happens");
                return TokenKind::Error("Expected delimiting `'` after character for literal.");
            }
        };

        TokenKind::Literal(TokenLiteralKind::Character(ch))
    }

    fn skip_whitespace_and_get_first_char(&mut self) -> Option<char> {
        while self.src.peek().is_some() {
            match self.src.peek().unwrap() {
                // todo: allow comments come after a statement
                // currently, comments have to be on their own lines
                '/' => {
                    self.advance();
                    match self.src.peek() {
                        Some(&ch) if ch == '/' => {
                            self.advance();

                            // advance until the end of the line.
                            self.advance_while(|ch| !is_newline(ch));
                            self.new_line();

                            // advance until a non-whitespace character is found.
                            self.skip_whitespace()
                        }
                        _ => return Some('/'),
                    }
                }
                &ch if is_newline(ch) => {
                    let ch = self.advance();

                    if !self.last_token_new_line {
                        return ch;
                    }
                    self.new_line();
                }
                &ch if is_whitespace(ch) => {
                    self.advance();
                }
                _ => return self.advance(),
            }
        }
        None
    }

    fn skip_whitespace(&mut self) {
        while self.src.peek().is_some_and(|&ch| is_whitespace(ch)) {
            let ch = self.advance().unwrap();
            if is_newline(ch) {
                self.new_line();
            }
        }
    }

    fn advance_while<C>(&mut self, condition: C)
    where
        C: Fn(char) -> bool,
    {
        while match self.src.peek() {
            Some(&ch) if condition(ch) => true,
            _ => false,
        } {
            self.advance();
        }
    }

    fn consume_while<C>(&mut self, condition: C, buffer: &mut String)
    where
        C: Fn(char) -> bool,
    {
        while condition(match self.src.peek() {
            Some(&c) => c,
            None => return,
        }) {
            buffer.push(self.advance().unwrap());
        }
    }
}

fn is_whitespace(ch: char) -> bool {
    ch.is_whitespace()
}

fn is_newline(ch: char) -> bool {
    ch == '\n'
}
