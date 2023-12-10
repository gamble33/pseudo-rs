use std::str::Chars;
use crate::lexer::token::{TokenKind::{self, *}, KeywordKind::*};

pub fn check_keyword(value: String) -> TokenKind {
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

fn check_ending(chars: Chars, rest: &str) -> bool {
    chars.collect::<String>() == rest
}
