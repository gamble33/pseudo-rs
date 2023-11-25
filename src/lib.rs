pub mod codegen_bytecode;
mod codegen_c;
pub mod ir;
pub mod lexer;
#[allow(dead_code, unused_variables)]
pub mod naive_tc;
pub mod parser;
#[allow(dead_code)]
mod vm;

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

    let hlir = naive_tc::typecheck(program);

    let chunk = codegen_bytecode::emit(hlir);

    vm::Vm::new().execute(chunk);
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
