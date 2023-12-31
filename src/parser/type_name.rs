use crate::lexer::token::{KeywordKind, Token, TokenKind};
use crate::error::ParseResult;
use crate::parser::Parser;
use crate::ir::ast::{BaseTypeName, TypeName};

impl<I> Parser<I>
    where I: Iterator<Item=Token>
{
    pub fn type_name(&mut self) -> ParseResult<TypeName> {
        if self.match_tokens(&[TokenKind::Keyword(KeywordKind::Array)]) {
            unimplemented!("parsing of ARRAY types.")
        } else {
            self.base_type_name()
        }
    }

    fn base_type_name(&mut self) -> ParseResult<TypeName> {
        match self.tokens.next() {
            Some(token) => match token.clone().kind {
                TokenKind::Keyword(keyword) => match keyword {
                    KeywordKind::Integer => Ok(TypeName::BaseTypeName(BaseTypeName::Integer)),
                    KeywordKind::Real => Ok(TypeName::BaseTypeName(BaseTypeName::Real)),
                    KeywordKind::Str => Ok(TypeName::BaseTypeName(BaseTypeName::String)),
                    KeywordKind::Char => Ok(TypeName::BaseTypeName(BaseTypeName::Char)),
                    KeywordKind::Boolean => Ok(TypeName::BaseTypeName(BaseTypeName::Boolean)),
                    KeywordKind::Date => Ok(TypeName::BaseTypeName(BaseTypeName::Date)),
                    _ => self.error(
                        "expected a base type like INTEGER or something.",
                        Some(token),
                    )
                }
                TokenKind::Identifier(name) => Ok(TypeName::BaseTypeName(BaseTypeName::Identifier(name))),
                _ => self.error(
                    "expected a base type like INTEGER or something.",
                    Some(token),
                )
            }
            None => self.error(
                "expected type name.",
                None,
            )
        }
    }
}
