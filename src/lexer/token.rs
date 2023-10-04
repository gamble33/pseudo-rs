#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
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

#[derive(Debug, PartialEq)]
pub enum TokenLiteralKind {
    Integer(i32),
    Character(char),
    String(String),
}

#[derive(Debug, PartialEq)]
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
    String,
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