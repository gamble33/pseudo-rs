mod lexer;
mod parser;
mod naive_tc;
mod codegen_c;
#[allow(dead_code)] mod vm;

use crate::codegen_c::generate;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn compile_to_c(src: &str) {
    let tokens = Lexer::new(src);

    let program = match Parser::new(tokens.peekable()).program() {
        Ok(decls) => decls,
        Err(errors) => {
            errors.iter().for_each(|error| {
                println!("error: {}", error.msg);
                match &error.token {
                    Some(token) => {
                        println!("got `{:?}`", error.token);
                    }
                    None => (),
                }
            });
            std::process::exit(0);
        }
    };

    let c_src = generate(program);

    std::fs::write("./target.c", c_src).expect("Unable to write to target C source file.");
}