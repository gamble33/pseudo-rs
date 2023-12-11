mod keyword;
#[cfg(test)]
mod tests;
pub mod token;

use crate::lexer::token::{Token, TokenKind, TokenLiteralKind};
use std::iter::Peekable;
use std::str::Chars;
use TokenKind::*;

pub struct Lexer<'a> {
    src: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src: src.chars().peekable(),
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
                        self.src.next();
                        NotEqual
                    }
                    '=' => {
                        self.src.next();
                        LessEqual
                    }
                    '-' => {
                        self.src.next();
                        LeftArrow
                    }
                    _ => Less,
                },
                None => Less,
            },
            '>' => match self.src.peek() {
                Some(c) => match c {
                    '=' => {
                        self.src.next();
                        GreaterEqual
                    }
                    _ => Greater,
                },
                None => Greater,
            },

            // todo: fix if new line isn't empty (contains space),
            // then its registered as another newline.
            ch if is_newline(ch) => {
                while match self.src.peek() {
                    Some(&c) if is_newline(c) => true,
                    _ => false,
                } {
                    self.src.next();
                }
                NewLine
            }

            '"' => self.string(),
            '\'' => self.character(),
            ch if ch.is_ascii_digit() => self.number(ch),
            ch if ch.is_alphabetic() => self.identifier(ch),

            _ => todo!("implement error handling for invalid token"),
        };

        Some(Token { kind: token_kind })
    }
}

impl Lexer<'_> {
    fn identifier(&mut self, first_char: char) -> TokenKind {
        let mut value = String::from(first_char);
        self.consume_while(|ch| ch.is_alphabetic() || ch.is_ascii_digit(), &mut value);

        keyword::check_keyword(value)
    }

    fn number(&mut self, first_char: char) -> TokenKind {
        let mut string_value = String::from(first_char);
        self.consume_while(|c| c.is_ascii_digit(), &mut string_value);

        if self.src.peek().is_some_and(|&ch| ch == '.') {
            string_value.push(self.src.next().unwrap());
            self.consume_while(|c| c.is_ascii_digit(), &mut string_value);
            let value = match string_value.parse::<f64>() {
                Ok(f) => f,
                Err(_) => unimplemented!("REAL value too large."),
            };
            TokenKind::Literal(TokenLiteralKind::Real(value))
        } else {
            let value = match string_value.parse::<i64>() {
                Ok(i) => i,
                Err(_) => unimplemented!("Implement integers being too big"),
            };
            TokenKind::Literal(TokenLiteralKind::Integer(value))
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
            todo!("unterminated string");
        }
        self.src.next();
        TokenKind::Literal(TokenLiteralKind::Str(value))
    }

    fn character(&mut self) -> TokenKind {
        let ch = match self.src.next() {
            Some(ch) => ch,
            None => {
                todo!("Expected character after `'`.");
            }
        };
        match self.src.next() {
            Some('\'') => (),
            _ => {
                todo!("Expected delimiting `'` after character for literal.");
            }
        };

        TokenKind::Literal(TokenLiteralKind::Character(ch))
    }

    fn skip_whitespace_and_get_first_char(&mut self) -> Option<char> {
        while self.src.peek().is_some() {
            match self.src.peek().unwrap() {
                '/' => {
                    self.src.next();
                    match self.src.peek() {
                        Some(&ch) if ch == '/' => {
                            self.src.next();

                            self.advance_while(|ch| !is_newline(ch));
                            self.advance_while(is_newline);
                        }
                        _ => return Some('/'),
                    }
                }
                &ch if is_whitespace(ch) => {
                    self.src.next();
                }
                _ => return self.src.next(),
            }
        }
        None
    }

    fn advance_while<C>(&mut self, condition: C)
    where
        C: Fn(char) -> bool,
    {
        while match self.src.peek() {
            Some(&ch) if condition(ch) => true,
            _ => false,
        } {
            self.src.next();
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
            buffer.push(self.src.next().unwrap());
        }
    }
}

fn is_whitespace(ch: char) -> bool {
    ch.is_whitespace() && ch != '\n' && ch != '\r'
}

fn is_newline(ch: char) -> bool {
    ch == '\n' || ch == '\r'
}
