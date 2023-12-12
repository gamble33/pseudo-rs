pub mod codegen_bytecode;
mod codegen_c;
pub mod ir;
#[allow(unreachable_code)] pub mod lexer;
pub mod naive_tc;
pub mod parser;
pub mod vm;

use crate::codegen_c::generate;
use crate::lexer::Lexer;
use crate::parser::program;

pub fn interpret(src: &str) {
    let tokens = Lexer::new(src);

    let program = match program(tokens.peekable()) {
        Ok(decls) => decls,
        Err(errors) => {
            parser::error::print_parse_errors(src, errors);
            std::process::exit(0);
        }
    };

    let hlir = naive_tc::typecheck(program);
    let mut vm = vm::Vm::new();
    let script = codegen_bytecode::emit(hlir, &mut vm);
    vm.execute(script);
}

pub fn compile_to_c(src: &str) {
    let tokens = Lexer::new(src);

    let program = match program(tokens.peekable()) {
        Ok(decls) => decls,
        Err(errors) => {
            parser::error::print_parse_errors(src, errors);
            std::process::exit(0);
        }
    };

    let c_src = generate(program);

    std::fs::write("./target.c", c_src).expect("Unable to write to target C source file.");
}
