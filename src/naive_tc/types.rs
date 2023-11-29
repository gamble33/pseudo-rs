use super::TypeChecker;
use crate::ir::{ast, ast::TypeName, hlir::Type};


impl TypeChecker {
    pub fn pseudo_type(&mut self, type_name: TypeName) -> Type {
        match type_name {
            TypeName::BaseTypeName(base_type_name) => match base_type_name {
                ast::BaseTypeName::Integer => Type::Integer,
                ast::BaseTypeName::Real => Type::Real,
                ast::BaseTypeName::String => Type::String,
                ast::BaseTypeName::Char => Type::Char,
                ast::BaseTypeName::Boolean => Type::Boolean,
                ast::BaseTypeName::Date => unimplemented!(),
                ast::BaseTypeName::Identifier(_) => unimplemented!(),
            },
        }
    }
}
