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
    generator.target.push_str("#define print(x) _Generic((x), \\\n");
    generator.target.push_str("    char: printf(\"%c\\n\", (x)), \\\n");
    generator.target.push_str("    int: printf(\"%d\\n\", (x)), \\\n");
    generator.target.push_str("    long: printf(\"%ld\\n\", (x)), \\\n");
    generator.target.push_str("    float: printf(\"%f\\n\", (x)), \\\n");
    generator.target.push_str("    double: printf(\"%lf\\n\", (x)), \\\n");
    generator.target.push_str("    default: printf(\"Unknown type\\n\") \\\n");
    generator.target.push_str(")\n");

    for decl in decls {
        generator.decl(decl);
    }

    generator.target
}

fn identifier(name: &str) -> String {
    match name {
        "Main" => String::from("main"),
        _ => String::from(format!("ident_{}", name))
    }
}
