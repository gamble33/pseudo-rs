pub mod lexer;
pub mod parser;
#[allow(dead_code, unused_variables)] mod naive_tc;
mod codegen_c;
mod codegen_bytecode;
#[allow(dead_code)] mod vm;

use crate::codegen_c::generate;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn interpret(src: &str) {
    let tokens = Lexer::new(src);

    let program = match Parser::new(tokens.peekable()).program() {
        Ok(decls) => decls,
        Err(errors) => {
            parser::print_parse_errors(errors);
            std::process::exit(0);
        }
    };

    let _chunk = codegen_bytecode::emit(program);

}

pub fn compile_to_c(src: &str) {
    let tokens = Lexer::new(src);

    let program = match Parser::new(tokens.peekable()).program() {
        Ok(decls) => decls,
        Err(errors) => {
            parser::print_parse_errors(errors);
            std::process::exit(0);
        }
    };

    let c_src = generate(program);

    std::fs::write("./target.c", c_src).expect("Unable to write to target C source file.");
}
