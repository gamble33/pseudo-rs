#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
    pub len: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, col: usize, len: usize) -> Self {
        Self {
            kind,
            line,
            col,
            len,
        }
    }

    pub fn from(kind: TokenKind) -> Self {
        Self {
            kind,
            line: 0,
            col: 0,
            len: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Error(&'static str),

    Identifier(String),
    Literal(TokenLiteralKind),
    Keyword(KeywordKind),

    NewLine,

    OpenSqrBracket,
    CloseSqrBracket,

    OpenParen,
    CloseParen,

    Colon,
    Comma,
    Caret,
    Dot,
    Ampersand,

    Plus,
    Minus,
    Slash,
    Star,

    LeftArrow,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    NotEqual,
    Equal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenLiteralKind {
    Integer(i64),
    Real(f64),
    Character(char),
    Str(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeywordKind {
    And,
    Or,
    Not,

    Div,
    Mod,

    Declare,
    Constant,

    Type,
    EndType,

    Integer,
    Real,
    Char,
    Str,
    Boolean,
    Date,

    Array,
    Of,

    ByRef,
    ByVal,

    Function,
    EndFunction,

    Return,
    Returns,

    Call,
    Procedure,
    EndProcedure,

    For,
    To,
    Step,
    Next,

    While,
    EndWhile,

    Repeat,
    Until,

    If,
    Then,
    Else,
    EndIf,

    True,
    False,

    Case,
    Otherwise,
    EndCase,

    // Built-in procedures

    Input,
    Output,

    OpenFile,
    ReadFile,
    WriteFile,
    CloseFile,
    Read,
    Write,
    Append,
    Random,

    // todo: GetRecord, Seek, PutRecord
}
