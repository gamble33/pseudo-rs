use crate::lexer::{KeywordKind, Lexer, TokenKind, TokenLiteralKind};
use TokenKind::*;

fn check_lexing(src: &str, expect: Vec<TokenKind>) {
    let actual: Vec<TokenKind> = Lexer::new(src).map(|t| t.kind).collect();
    assert_eq!(actual, expect);
}

#[test]
fn smoke_test() {
    use KeywordKind::*;
    check_lexing(
        "PROCEDURE Main
            OUTPUT \"Hello, World!\"
         ENDPROCEDURE",
        vec![
            Keyword(Procedure),
            Identifier(String::from("Main")),
            NewLine,
            Keyword(Output),
            Literal(TokenLiteralKind::Str(String::from("Hello, World!"))),
            NewLine,
            Keyword(EndProcedure),
        ],
    );
}

#[test]
fn characters() {
    check_lexing(
        "'a' ' ' '9'",
        vec![
            Literal(TokenLiteralKind::Character('a')),
            Literal(TokenLiteralKind::Character(' ')),
            Literal(TokenLiteralKind::Character('9')),
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
            Keyword(KeywordKind::Procedure),
            NewLine,
        ],
    );
}

#[test]
fn new_lines() {
    check_lexing(
        "line1\r\nline2\nline3\rline4",
        vec![
            Identifier(String::from("line1")),
            NewLine,
            Identifier(String::from("line2")),
            NewLine,
            Identifier(String::from("line3")),
            NewLine,
            Identifier(String::from("line4")),
        ],
    );
}

#[test]
fn multiple_new_lines() {
    check_lexing(
        "line1\r\n\r\n\r\nline2",
        vec![
            Identifier(String::from("line1")),
            NewLine,
            Identifier(String::from("line2")),
        ],
    );
    check_lexing(
        "line1\r\r\r\r\r\r\n\r\n\n\n\r\n\n\n\n\r\n\n\r\rline2",
        vec![
            Identifier(String::from("line1")),
            NewLine,
            Identifier(String::from("line2")),
        ],
    );
}

#[test]
fn keywords() {
    use KeywordKind::*;
    check_lexing("AND", vec![Keyword(And)]);
    check_lexing("OR", vec![Keyword(Or)]);
    check_lexing("NOT", vec![Keyword(Not)]);
    check_lexing("DIV", vec![Keyword(Div)]);
    check_lexing("MOD", vec![Keyword(Mod)]);
    check_lexing("DECLARE", vec![Keyword(Declare)]);
    check_lexing("CONSTANT", vec![Keyword(Constant)]);
    check_lexing("TYPE", vec![Keyword(Type)]);
    check_lexing("ENDTYPE", vec![Keyword(EndType)]);
    check_lexing("INTEGER", vec![Keyword(Integer)]);
    check_lexing("REAL", vec![Keyword(Real)]);
    check_lexing("CHAR", vec![Keyword(Char)]);
    check_lexing("STRING", vec![Keyword(Str)]);
    check_lexing("BOOLEAN", vec![Keyword(Boolean)]);
    check_lexing("DATE", vec![Keyword(Date)]);
    check_lexing("ARRAY", vec![Keyword(Array)]);
    check_lexing("OF", vec![Keyword(Of)]);
    check_lexing("BYREF", vec![Keyword(ByRef)]);
    check_lexing("BYVAL", vec![Keyword(ByVal)]);
    check_lexing("FUNCTION", vec![Keyword(Function)]);
    check_lexing(
        "ENDFUNCTION",
        vec![Keyword(EndFunction)],
    );
    check_lexing("RETURN", vec![Keyword(Return)]);
    check_lexing("RETURNS", vec![Keyword(Returns)]);
    check_lexing("CALL", vec![Keyword(Call)]);
    check_lexing(
        "PROCEDURE",
        vec![Keyword(Procedure)],
    );
    check_lexing(
        "ENDPROCEDURE",
        vec![Keyword(EndProcedure)],
    );
    check_lexing("FOR", vec![Keyword(For)]);
    check_lexing("TO", vec![Keyword(To)]);
    check_lexing("STEP", vec![Keyword(Step)]);
    check_lexing("NEXT", vec![Keyword(Next)]);
    check_lexing("WHILE", vec![Keyword(While)]);
    check_lexing("ENDWHILE", vec![Keyword(EndWhile)]);
    check_lexing("REPEAT", vec![Keyword(Repeat)]);
    check_lexing("UNTIL", vec![Keyword(Until)]);
    check_lexing("IF", vec![Keyword(If)]);
    check_lexing("THEN", vec![Keyword(Then)]);
    check_lexing("ELSE", vec![Keyword(Else)]);
    check_lexing("ENDIF", vec![Keyword(EndIf)]);
    check_lexing("TRUE", vec![Keyword(True)]);
    check_lexing("FALSE", vec![Keyword(False)]);
    check_lexing("CASE", vec![Keyword(Case)]);
    check_lexing(
        "OTHERWISE",
        vec![Keyword(Otherwise)],
    );
    check_lexing("ENDCASE", vec![Keyword(EndCase)]);
    check_lexing("INPUT", vec![Keyword(Input)]);
    check_lexing("OUTPUT", vec![Keyword(Output)]);
    check_lexing("OPENFILE", vec![Keyword(OpenFile)]);
    check_lexing("READFILE", vec![Keyword(ReadFile)]);
    check_lexing(
        "WRITEFILE",
        vec![Keyword(WriteFile)],
    );
    check_lexing(
        "CLOSEFILE",
        vec![Keyword(CloseFile)],
    );
    check_lexing("READ", vec![Keyword(Read)]);
    check_lexing("WRITE", vec![Keyword(Write)]);
    check_lexing("APPEND", vec![Keyword(Append)]);
    check_lexing("RANDOM", vec![Keyword(Random)]);
}
