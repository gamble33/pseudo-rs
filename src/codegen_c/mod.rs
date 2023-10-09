mod decl;
mod stmt;
mod type_name;
mod expr;

use crate::parser::stmt::{Decl, Stmt};

struct Generator {
    target: String,
}

pub fn generate(decls: Vec<Decl>) -> String {
    let mut generator = Generator {
        target: String::new()
    };

    for decl in decls {
        generator.decl(decl);
    }

    generator.target
}

fn identifier(name: String) -> String {
    String::from(format!("ident_{}", name))
}