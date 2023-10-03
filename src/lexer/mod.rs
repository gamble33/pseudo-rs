pub mod token;

use std::str::Chars;
use std::iter::Peekable;
use crate::lexer::token::{KeywordKind, TokenLiteralKind, Token, TokenKind};

pub struct Lexer<'a> {
    src: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            src: src.chars().peekable()
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
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '.' => TokenKind::Dot,
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '[' => TokenKind::OpenSqrBracket,
            ']' => TokenKind::CloseSqrBracket,
            '=' => TokenKind::Equal,
            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,
            '^' => TokenKind::Caret,

            '<' => match self.src.peek() {
                Some(c) => match c {
                    '>' => {
                        self.src.next();
                        TokenKind::NotEqual
                    }
                    '=' => {
                        self.src.next();
                        TokenKind::LessEqual
                    }
                    '-' => {
                        self.src.next();
                        TokenKind::LeftArrow
                    }
                    _ => TokenKind::Less,
                }
                None => TokenKind::Less
            },
            '>' => match self.src.peek() {
                Some(c) => match c {
                    '=' => {
                        self.src.next();
                        TokenKind::GreaterEqual
                    }
                    _ => TokenKind::Greater,
                }
                None => TokenKind::Greater
            }

            ch if is_newline(ch) => {
                while match self.src.peek() {
                    Some(&c) if is_newline(c) => true,
                    _ => false
                } {
                    self.src.next();
                }
                TokenKind::NewLine
            }

            '"' => self.string(),
            '\'' => self.character(),
            ch if ch.is_ascii_digit() => self.number(ch),
            ch if ch.is_alphabetic() => self.identifier(ch),

            _ => todo!("implement error handling for invalid token"),
        };

        Some(Token {
            kind: token_kind,
        })
    }
}

impl Lexer<'_> {
    fn identifier(&mut self, first_char: char) -> TokenKind {
        let mut value = String::from(first_char);
        self.consume_while(
            |ch| ch.is_alphabetic() || ch.is_ascii_digit(),
            &mut value,
        );
        let chars = value.chars();

        // todo: OPTIMISE by letting `Chars` iterator be consumed and each branch and (in the end) comparing with what's left.

        match chars.clone().nth(0).unwrap() {
            'A' => match chars.clone().nth(1) {
                Some('N') => if check_keyword(chars.clone(), 2, "D") { return TokenKind::Keyword(KeywordKind::And); },
                Some('R') => if check_keyword(chars.clone(), 2, "RAY") { return TokenKind::Keyword(KeywordKind::Array); },
                Some('P') => if check_keyword(chars.clone(), 2, "PEND") { return TokenKind::Keyword(KeywordKind::Append); },
                _ => ()
            }
            'B' => match chars.clone().nth(1) {
                Some('O') => if check_keyword(chars.clone(), 2, "OLEAN") { return TokenKind::Keyword(KeywordKind::Boolean); },
                Some('Y') => match chars.clone().nth(2) {
                    Some('R') => if check_keyword(chars.clone(), 3, "EF") { return TokenKind::Keyword(KeywordKind::ByRef); },
                    Some('V') => if check_keyword(chars.clone(), 3, "VAL") { return TokenKind::Keyword(KeywordKind::ByVal); },
                    _ => ()
                }
                _ => ()
            },
            'C' => match chars.clone().nth(1) {
                Some('O') => if check_keyword(chars.clone(), 2, "NSTANT") { return TokenKind::Keyword(KeywordKind::Constant); },
                Some('H') => if check_keyword(chars.clone(), 2, "AR") { return TokenKind::Keyword(KeywordKind::Char); },
                Some('L') => if check_keyword(chars.clone(), 2, "OSEFILE") { return TokenKind::Keyword(KeywordKind::CloseFile); },
                Some('A') => match chars.clone().nth(2) {
                    Some('L') => if check_keyword(chars.clone(), 3, "L") { return TokenKind::Keyword(KeywordKind::Call); },
                    Some('S') => if check_keyword(chars.clone(), 3, "E") { return TokenKind::Keyword(KeywordKind::Case); },
                    _ => ()
                },
                _ => ()
            },
            'D' => match chars.clone().nth(1) {
                Some('A') => if check_keyword(chars.clone(), 2, "TE") { return TokenKind::Keyword(KeywordKind::Date); },
                Some('E') => if check_keyword(chars.clone(), 2, "CLARE") { return TokenKind::Keyword(KeywordKind::Declare); },
                Some('I') => if check_keyword(chars.clone(), 2, "V") { return TokenKind::Keyword(KeywordKind::Div); },
                _ => ()
            },
            'E' => match chars.clone().nth(1) {
                Some('L') => if check_keyword(chars.clone(), 2, "SE") { return TokenKind::Keyword(KeywordKind::Else); },
                Some('N') => match chars.clone().nth(2) {
                    Some('D') => match chars.clone().nth(3) {
                        Some('C') => if check_keyword(chars.clone(), 4, "ASE") { return TokenKind::Keyword(KeywordKind::EndCase); },
                        Some('T') => if check_keyword(chars.clone(), 4, "YPE") { return TokenKind::Keyword(KeywordKind::EndType); },
                        Some('F') => if check_keyword(chars.clone(), 4, "UNCTION") { return TokenKind::Keyword(KeywordKind::EndFunction); },
                        Some('P') => if check_keyword(chars.clone(), 4, "ROCEDURE") { return TokenKind::Keyword(KeywordKind::EndProcedure); },
                        Some('W') => if check_keyword(chars.clone(), 4, "HILE") { return TokenKind::Keyword(KeywordKind::EndWhile); },
                        Some('I') => if check_keyword(chars.clone(), 4, "F") { return TokenKind::Keyword(KeywordKind::EndIf); },
                        _ => ()
                    }
                    _ => ()
                }
                _ => ()
            },
            'F' => match chars.clone().nth(1) {
                Some('A') => if check_keyword(chars.clone(), 2, "LSE") { return TokenKind::Keyword(KeywordKind::False); },
                Some('O') => if check_keyword(chars.clone(), 2, "R") { return TokenKind::Keyword(KeywordKind::For); },
                Some('U') => if check_keyword(chars.clone(), 2, "NCTION") { return TokenKind::Keyword(KeywordKind::Function); },
                _ => ()
            },
            'I' => match chars.clone().nth(1) {
                Some('F') => if check_keyword(chars.clone(), 2, "") { return TokenKind::Keyword(KeywordKind::If); },
                Some('N') => match chars.clone().nth(2) {
                    Some('P') => if check_keyword(chars.clone(), 3, "UT") { return TokenKind::Keyword(KeywordKind::Input); },
                    Some('T') => if check_keyword(chars.clone(), 3, "EGER") { return TokenKind::Keyword(KeywordKind::Integer); },
                    _ => ()
                }
                _ => ()
            },
            'M' => if check_keyword(chars.clone(), 1, "OD") { return TokenKind::Keyword(KeywordKind::Mod); },
            'N' => match chars.clone().nth(1) {
                Some('E') => if check_keyword(chars.clone(), 2, "XT") { return TokenKind::Keyword(KeywordKind::Next); },
                Some('O') => if check_keyword(chars.clone(), 2, "T") { return TokenKind::Keyword(KeywordKind::Not); },
                _ => ()
            },
            'O' => match chars.clone().nth(1) {
                Some('R') => if check_keyword(chars.clone(), 2, "") { return TokenKind::Keyword(KeywordKind::Or); }
                Some('F') => if check_keyword(chars.clone(), 2, "") { return TokenKind::Keyword(KeywordKind::Of); }
                Some('T') => if check_keyword(chars.clone(), 2, "HERWISE") { return TokenKind::Keyword(KeywordKind::Otherwise); },
                Some('U') => if check_keyword(chars.clone(), 2, "TPUT") { return TokenKind::Keyword(KeywordKind::Output); },
                Some('P') => if check_keyword(chars.clone(), 2, "ENFILE") { return TokenKind::Keyword(KeywordKind::OpenFile); },
                _ => ()
            },
            'P' => if check_keyword(chars.clone(), 1, "ROCEDURE") { return TokenKind::Keyword(KeywordKind::Procedure); }
            'R' => match chars.clone().nth(1) {
                Some('A') => if check_keyword(chars.clone(), 2, "NDOM") { return TokenKind::Keyword(KeywordKind::Random); },

                Some('E') => match chars.clone().nth(2) {
                    Some('T') => match chars.clone().nth(3) {
                        Some('U') => match chars.clone().nth(4) {
                            Some('R') => match chars.clone().nth(5) {
                                Some('N') => match chars.clone().nth(6) {
                                    Some('S') => if check_keyword(chars.clone(), 7, "") { return TokenKind::Keyword(KeywordKind::Returns); },
                                    None => { return TokenKind::Keyword(KeywordKind::Return); }
                                    _ => ()
                                },
                                _ => ()
                            },
                            _ => ()
                        },
                        _ => ()
                    },

                    Some('P') => if check_keyword(chars.clone(), 3, "EAT") { return TokenKind::Keyword(KeywordKind::Repeat); },

                    Some('A') => match chars.clone().nth(3) {
                        Some('L') => if check_keyword(chars.clone(), 2, "") { return TokenKind::Keyword(KeywordKind::Real); },
                        Some('D') => match chars.clone().nth(4) {
                            Some('F') => if check_keyword(chars.clone(), 5, "ILE") { return TokenKind::Keyword(KeywordKind::ReadFile); },
                            None => if check_keyword(chars.clone(), 5, "") { return TokenKind::Keyword(KeywordKind::Read); },
                            _ => ()
                        },
                        _ => ()
                    },
                    _ => ()
                },
                _ => ()
            },
            'S' => if check_keyword(chars.clone(), 1, "TRING") { return TokenKind::Keyword(KeywordKind::String); },
            'T' => match chars.clone().nth(1) {
                Some('R') => if check_keyword(chars.clone(), 2, "UE") { return TokenKind::Keyword(KeywordKind::True); },
                Some('O') => if check_keyword(chars.clone(), 2, "") { return TokenKind::Keyword(KeywordKind::To); },
                Some('H') => if check_keyword(chars.clone(), 2, "EN") { return TokenKind::Keyword(KeywordKind::Then); },
                Some('Y') => if check_keyword(chars.clone(), 2, "PE") { return TokenKind::Keyword(KeywordKind::Type); },
                _ => ()
            }
            'U' => if check_keyword(chars.clone(), 1, "NTIL") { return TokenKind::Keyword(KeywordKind::Until); },
            'W' => match chars.clone().nth(1) {
                Some('H') => if check_keyword(chars.clone(), 2, "ILE") { return TokenKind::Keyword(KeywordKind::While); },
                Some('R') => match chars.clone().nth(2) {
                    Some('I') => match chars.clone().nth(3) {
                        Some('T') => match chars.clone().nth(4) {
                            Some('E') => match chars.clone().nth(5) {
                                Some('F') => if check_keyword(chars.clone(), 6, "ILE") { return TokenKind::Keyword(KeywordKind::WriteFile); },
                                None => if check_keyword(chars.clone(), 5, "") { return TokenKind::Keyword(KeywordKind::Write); }
                                _ => ()
                            },
                            _ => ()
                        },
                        _ => ()
                    }
                    _ => ()
                },
                _ => ()
            }
            _ => ()
        }

        TokenKind::Identifier(value)
    }

    fn number(&mut self, first_char: char) -> TokenKind {
        let mut string_value = String::from(first_char);
        self.consume_while(
            |c| c.is_ascii_digit(),
            &mut string_value,
        );
        let value = match string_value.parse::<i32>() {
            Ok(i) => i,
            Err(_) => todo!("Implement integers being too big"),
        };

        // todo: implement REAL literals

        TokenKind::Literal(TokenLiteralKind::Integer(value))
    }

    fn string(&mut self) -> TokenKind {
        let mut value = String::new();
        self.consume_while(|ch| ch != '"' && !is_newline(ch), &mut value);
        if self.src.peek().is_none() || match self.src.peek() {
            Some(&c) if c != '"' => true,
            _ => false,
        } {
            todo!("unterminated string");
        }
        self.src.next();
        TokenKind::Literal(TokenLiteralKind::String(value))
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

                            while match self.src.peek() {
                                Some(&ch) if !is_newline(ch) => true,
                                _ => false
                            } {
                                self.next();
                            }

                            while match self.src.peek() {
                                Some(&ch) if is_newline(ch) => true,
                                _ => false
                            } {
                                self.next();
                            }
                        }
                        _ => return Some('/')
                    }
                }
                &ch if is_whitespace(ch) => { self.src.next(); }
                _ => return self.src.next()
            }
        }
        None
    }

    fn consume_while<C>(&mut self, condition: C, buffer: &mut String)
        where C: Fn(char) -> bool
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

fn check_keyword(chars: Chars, start: usize, rest: &str) -> bool {
    chars.clone().skip(start).take(chars.count() - start).collect::<String>() == rest
}