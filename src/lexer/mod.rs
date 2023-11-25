pub mod token;
#[cfg(test)]
mod tests;

use std::str::Chars;
use std::iter::Peekable;
use crate::lexer::token::{KeywordKind, TokenLiteralKind, Token, TokenKind};
use TokenKind::*;

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
                }
                None => Less
            },
            '>' => match self.src.peek() {
                Some(c) => match c {
                    '=' => {
                        self.src.next();
                        GreaterEqual
                    }
                    _ => Greater,
                }
                None => Greater
            }

            ch if is_newline(ch) => {
                while match self.src.peek() {
                    Some(&c) if is_newline(c) => true,
                    _ => false
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

        Some(Token {
            kind: token_kind,
        })
    }
}

impl Lexer<'_> {
    fn identifier(&mut self, first_char: char) -> TokenKind {
        use KeywordKind::*;

        let mut value = String::from(first_char);
        self.consume_while(
            |ch| ch.is_alphabetic() || ch.is_ascii_digit(),
            &mut value,
        );
        let mut chars = value.chars();

        match chars.next() {
            Some('A') => match chars.next() {
              Some('N') => if check_ending(chars, "D") { return Keyword(And); },
                Some('R') => if check_ending(chars, "RAY") { return Keyword(Array); },
                Some('P') => if check_ending(chars, "PEND") { return Keyword(Append); },
                _ => ()
            }
            Some('B') => match chars.next() {
                Some('O') => if check_ending(chars, "OLEAN") { return Keyword(Boolean); },
                Some('Y') => match chars.next() {
                    Some('R') => if check_ending(chars, "EF") { return Keyword(ByRef); },
                    Some('V') => if check_ending(chars, "AL") { return Keyword(ByVal); },
                    _ => ()
                }
                _ => ()
            },
            Some('C') => match chars.next() {
                Some('O') => if check_ending(chars, "NSTANT") { return Keyword(Constant); },
                Some('H') => if check_ending(chars, "AR") { return Keyword(Char); },
                Some('L') => if check_ending(chars, "OSEFILE") { return Keyword(CloseFile); },
                Some('A') => match chars.next() {
                    Some('L') => if check_ending(chars, "L") { return Keyword(Call); },
                    Some('S') => if check_ending(chars, "E") { return Keyword(Case); },
                    _ => ()
                },
                _ => ()
            },
            Some('D') => match chars.next() {
                Some('A') => if check_ending(chars, "TE") { return Keyword(Date); },
                Some('E') => if check_ending(chars, "CLARE") { return Keyword(Declare); },
                Some('I') => if check_ending(chars, "V") { return Keyword(Div); },
                _ => ()
            },
            Some('E') => match chars.next() {
                Some('L') => if check_ending(chars, "SE") { return Keyword(Else); },
                Some('N') => match chars.next() {
                    Some('D') => match chars.next() {
                        Some('C') => if check_ending(chars, "ASE") { return Keyword(EndCase); },
                        Some('T') => if check_ending(chars, "YPE") { return Keyword(EndType); },
                        Some('F') => if check_ending(chars, "UNCTION") { return Keyword(EndFunction); },
                        Some('P') => if check_ending(chars, "ROCEDURE") { return Keyword(EndProcedure); },
                        Some('W') => if check_ending(chars, "HILE") { return Keyword(EndWhile); },
                        Some('I') => if check_ending(chars, "F") { return Keyword(EndIf); },
                        _ => ()
                    }
                    _ => ()
                }
                _ => ()
            },
            Some('F') => match chars.next() {
                Some('A') => if check_ending(chars, "LSE") { return Keyword(False); },
                Some('O') => if check_ending(chars, "R") { return Keyword(For); },
                Some('U') => if check_ending(chars, "NCTION") { return Keyword(Function); },
                _ => ()
            },
            Some('I') => match chars.next() {
                Some('F') => if check_ending(chars, "") { return Keyword(If); },
                Some('N') => match chars.next() {
                    Some('P') => if check_ending(chars, "UT") { return Keyword(Input); },
                    Some('T') => if check_ending(chars, "EGER") { return Keyword(Integer); },
                    _ => ()
                }
                _ => ()
            },
            Some('M') => if check_ending(chars, "OD") { return Keyword(Mod); },
            Some('N') => match chars.next() {
                Some('E') => if check_ending(chars, "XT") { return Keyword(Next); },
                Some('O') => if check_ending(chars, "T") { return Keyword(Not); },
                _ => ()
            },
            Some('O') => match chars.next() {
                Some('R') => if check_ending(chars, "") { return Keyword(Or); }
                Some('F') => if check_ending(chars, "") { return Keyword(Of); }
                Some('T') => if check_ending(chars, "HERWISE") { return Keyword(Otherwise); },
                Some('U') => if check_ending(chars, "TPUT") { return Keyword(Output); },
                Some('P') => if check_ending(chars, "ENFILE") { return Keyword(OpenFile); },
                _ => ()
            },
            Some('P') => if check_ending(chars, "ROCEDURE") { return Keyword(Procedure); }
            Some('R') => match chars.next() {
                Some('A') => if check_ending(chars, "NDOM") { return Keyword(Random); },

                Some('E') => match chars.next() {
                    Some('A') => match chars.next() {
                        Some('D') => match chars.next() {
                            Some('F') => if check_ending(chars, "ILE") { return Keyword(ReadFile); },
                            None => { return Keyword(Read); },
                            _ => ()
                                                    },
                        Some('L') => if check_ending(chars, "") { return Keyword(Real); },
                        _ => ()
                    },
                    Some('T') => match chars.next() {
                        Some('U') => match chars.next() {
                            Some('R') => match chars.next() {
                                Some('N') => match chars.next() {
                                    Some('S') => if check_ending(chars, "") { return Keyword(Returns); },
                                    None => { return Keyword(Return); }
                                    _ => ()
                                },
                                _ => ()
                            },
                            _ => ()
                        },
                        _ => ()
                    },

                    Some('P') => if check_ending(chars, "EAT") { return Keyword(Repeat); },
                    _ => ()
                },
                _ => ()
            },
            Some('S') => match chars.next() {
                Some('T') => match chars.next() {
                    Some('R') if check_ending(chars.clone(), "ING") => { return Keyword(Str); }
                    Some('E') if check_ending(chars, "P") => { return Keyword(Step); }
                    _ => ()
                },
                _ => ()
            },
            Some('T') => match chars.next() {
                Some('R') => if check_ending(chars, "UE") { return Keyword(True); },
                Some('O') => if check_ending(chars, "") { return Keyword(To); },
                Some('H') => if check_ending(chars, "EN") { return Keyword(Then); },
                Some('Y') => if check_ending(chars, "PE") { return Keyword(Type); },
                _ => ()
            }
            Some('U') => if check_ending(chars, "NTIL") { return Keyword(Until); },
            Some('W') => match chars.next() {
                Some('H') => if check_ending(chars, "ILE") { return Keyword(While); },
                Some('R') => match chars.next() {
                    Some('I') => match chars.next() {
                        Some('T') => match chars.next() {
                            Some('E') => match chars.next() {
                                Some('F') => if check_ending(chars, "ILE") { return Keyword(WriteFile); },
                                None => if check_ending(chars, "") { return Keyword(Write); }
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
                        _ => return Some('/')
                    }
                }
                &ch if is_whitespace(ch) => { self.src.next(); }
                _ => return self.src.next()
            }
        }
        None
    }

    fn advance_while<C>(&mut self, condition: C)
        where C: Fn(char) -> bool
    {
        while match self.src.peek() {
            Some(&ch) if condition(ch) => true,
            _ => false,
        } {
            self.src.next();
        }
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

fn check_ending(chars: Chars, rest: &str) -> bool {
    chars.collect::<String>() == rest
}
