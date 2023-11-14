use crate::lexer::{KeywordKind, Lexer, TokenKind, TokenLiteralKind};

fn check_lexing(src: &str, expect: Vec<TokenKind>) {
    let actual: Vec<TokenKind> = Lexer::new(src).map(|t| t.kind).collect();
    assert_eq!(actual, expect);
}

#[test]
fn smoke_test() {
    check_lexing(
        "PROCEDURE Main
            OUTPUT \"Hello, World!\"
         ENDPROCEDURE",
        vec![
            TokenKind::Keyword(KeywordKind::Procedure),
            TokenKind::Identifier(String::from("Main")),
            TokenKind::NewLine,
            TokenKind::Keyword(KeywordKind::Output),
            TokenKind::Literal(TokenLiteralKind::Str(String::from("Hello, World!"))),
            TokenKind::NewLine,
            TokenKind::Keyword(KeywordKind::EndProcedure),
        ],
    );
}

#[test]
fn characters() {
    check_lexing(
        "'a' ' ' '9'",
        vec![
            TokenKind::Literal(TokenLiteralKind::Character('a')),
            TokenKind::Literal(TokenLiteralKind::Character(' ')),
            TokenKind::Literal(TokenLiteralKind::Character('9')),
        ],
    );
}

#[test]
fn comments() {
    check_lexing("// Comment", vec![]);
    check_lexing(
        "// Comment 1
         // Multiple comments
         // Comment 3",
        vec![],
    );
    check_lexing(
        "// Comment 1
         PROCEDURE
         // Comment 3",
        vec![
            TokenKind::Keyword(KeywordKind::Procedure),
            TokenKind::NewLine,
        ],
    );
}

#[test]
fn new_lines() {
    check_lexing(
        "line1\r\nline2\nline3\rline4",
        vec![
            TokenKind::Identifier(String::from("line1")),
            TokenKind::NewLine,
            TokenKind::Identifier(String::from("line2")),
            TokenKind::NewLine,
            TokenKind::Identifier(String::from("line3")),
            TokenKind::NewLine,
            TokenKind::Identifier(String::from("line4")),
        ],
    );
}

#[test]
fn multiple_new_lines() {
    check_lexing(
        "line1\r\n\r\n\r\nline2",
        vec![
            TokenKind::Identifier(String::from("line1")),
            TokenKind::NewLine,
            TokenKind::Identifier(String::from("line2")),
        ],
    );
    check_lexing(
        "line1\r\r\r\r\r\r\n\r\n\n\n\r\n\n\n\n\r\n\n\r\rline2",
        vec![
            TokenKind::Identifier(String::from("line1")),
            TokenKind::NewLine,
            TokenKind::Identifier(String::from("line2")),
        ],
    );
}

#[test]
fn keywords() {
    check_lexing("AND", vec![TokenKind::Keyword(KeywordKind::And)]);
    check_lexing("OR", vec![TokenKind::Keyword(KeywordKind::Or)]);
    check_lexing("NOT", vec![TokenKind::Keyword(KeywordKind::Not)]);
    check_lexing("DIV", vec![TokenKind::Keyword(KeywordKind::Div)]);
    check_lexing("MOD", vec![TokenKind::Keyword(KeywordKind::Mod)]);
    check_lexing("DECLARE", vec![TokenKind::Keyword(KeywordKind::Declare)]);
    check_lexing("CONSTANT", vec![TokenKind::Keyword(KeywordKind::Constant)]);
    check_lexing("TYPE", vec![TokenKind::Keyword(KeywordKind::Type)]);
    check_lexing("ENDTYPE", vec![TokenKind::Keyword(KeywordKind::EndType)]);
    check_lexing("INTEGER", vec![TokenKind::Keyword(KeywordKind::Integer)]);
    check_lexing("REAL", vec![TokenKind::Keyword(KeywordKind::Real)]);
    check_lexing("CHAR", vec![TokenKind::Keyword(KeywordKind::Char)]);
    check_lexing("STRING", vec![TokenKind::Keyword(KeywordKind::Str)]);
    check_lexing("BOOLEAN", vec![TokenKind::Keyword(KeywordKind::Boolean)]);
    check_lexing("DATE", vec![TokenKind::Keyword(KeywordKind::Date)]);
    check_lexing("ARRAY", vec![TokenKind::Keyword(KeywordKind::Array)]);
    check_lexing("OF", vec![TokenKind::Keyword(KeywordKind::Of)]);
    check_lexing("BYREF", vec![TokenKind::Keyword(KeywordKind::ByRef)]);
    check_lexing("BYVAL", vec![TokenKind::Keyword(KeywordKind::ByVal)]);
    check_lexing("FUNCTION", vec![TokenKind::Keyword(KeywordKind::Function)]);
    check_lexing(
        "ENDFUNCTION",
        vec![TokenKind::Keyword(KeywordKind::EndFunction)],
    );
    check_lexing("RETURN", vec![TokenKind::Keyword(KeywordKind::Return)]);
    check_lexing("RETURNS", vec![TokenKind::Keyword(KeywordKind::Returns)]);
    check_lexing("CALL", vec![TokenKind::Keyword(KeywordKind::Call)]);
    check_lexing(
        "PROCEDURE",
        vec![TokenKind::Keyword(KeywordKind::Procedure)],
    );
    check_lexing(
        "ENDPROCEDURE",
        vec![TokenKind::Keyword(KeywordKind::EndProcedure)],
    );
    check_lexing("FOR", vec![TokenKind::Keyword(KeywordKind::For)]);
    check_lexing("TO", vec![TokenKind::Keyword(KeywordKind::To)]);
    check_lexing("STEP", vec![TokenKind::Keyword(KeywordKind::Step)]);
    check_lexing("NEXT", vec![TokenKind::Keyword(KeywordKind::Next)]);
    check_lexing("WHILE", vec![TokenKind::Keyword(KeywordKind::While)]);
    check_lexing("ENDWHILE", vec![TokenKind::Keyword(KeywordKind::EndWhile)]);
    check_lexing("REPEAT", vec![TokenKind::Keyword(KeywordKind::Repeat)]);
    check_lexing("UNTIL", vec![TokenKind::Keyword(KeywordKind::Until)]);
    check_lexing("IF", vec![TokenKind::Keyword(KeywordKind::If)]);
    check_lexing("THEN", vec![TokenKind::Keyword(KeywordKind::Then)]);
    check_lexing("ELSE", vec![TokenKind::Keyword(KeywordKind::Else)]);
    check_lexing("ENDIF", vec![TokenKind::Keyword(KeywordKind::EndIf)]);
    check_lexing("TRUE", vec![TokenKind::Keyword(KeywordKind::True)]);
    check_lexing("FALSE", vec![TokenKind::Keyword(KeywordKind::False)]);
    check_lexing("CASE", vec![TokenKind::Keyword(KeywordKind::Case)]);
    check_lexing(
        "OTHERWISE",
        vec![TokenKind::Keyword(KeywordKind::Otherwise)],
    );
    check_lexing("ENDCASE", vec![TokenKind::Keyword(KeywordKind::EndCase)]);
    check_lexing("INPUT", vec![TokenKind::Keyword(KeywordKind::Input)]);
    check_lexing("OUTPUT", vec![TokenKind::Keyword(KeywordKind::Output)]);
    check_lexing("OPENFILE", vec![TokenKind::Keyword(KeywordKind::OpenFile)]);
    check_lexing("READFILE", vec![TokenKind::Keyword(KeywordKind::ReadFile)]);
    check_lexing(
        "WRITEFILE",
        vec![TokenKind::Keyword(KeywordKind::WriteFile)],
    );
    check_lexing(
        "CLOSEFILE",
        vec![TokenKind::Keyword(KeywordKind::CloseFile)],
    );
    check_lexing("READ", vec![TokenKind::Keyword(KeywordKind::Read)]);
    check_lexing("WRITE", vec![TokenKind::Keyword(KeywordKind::Write)]);
    check_lexing("APPEND", vec![TokenKind::Keyword(KeywordKind::Append)]);
    check_lexing("RANDOM", vec![TokenKind::Keyword(KeywordKind::Random)]);
}
