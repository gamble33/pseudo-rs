mod decl;
mod stmt;
mod type_name;
mod expr;

use crate::parser::stmt::Decl;

struct Generator {
    target: String,
}

pub fn generate(decls: Vec<Decl>) -> String {
    let mut generator = Generator {
        target: String::new()
    };

    generator.target.push_str("#include <stdio.h>\n");
    generator.target.push_str("#include <stdbool.h>\n");
    generator.target.push_str("#define print(x) _Generic(x)\n");

    for decl in decls {
        generator.decl(decl);
    }

    generator.target
}

fn identifier(name: &str) -> String {
    String::from(format!("ident_{}", name))
}