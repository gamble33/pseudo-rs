use crate::codegen_c::{Generator, identifier};
use crate::parser::type_name::{BaseTypeName, TypeName};

impl Generator {
    pub fn type_name(&mut self, type_name: &TypeName) {
        match type_name {
            TypeName::BaseTypeName(base_type_name) => match base_type_name  {
                BaseTypeName::Integer => {
                    self.target.push_str("int ");
                },
                BaseTypeName::Real => {
                    self.target.push_str("float ");
                },
                BaseTypeName::String => unimplemented!(),

                BaseTypeName::Char => {
                    self.target.push_str("char ");
                },
                BaseTypeName::Boolean => {
                    self.target.push_str("bool ");
                },
                BaseTypeName::Date => unimplemented!(),

                BaseTypeName::Identifier(name) => {
                    self.target.push_str(&format!("{} ", identifier(name)));
                },
            }
        }
    }
}