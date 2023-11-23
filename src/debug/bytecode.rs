use pseudo_rs::{parser, parser::Parser, lexer::Lexer, codegen_bytecode::emit};

pub fn print_bytecode(src: &str) {
   emit(match Parser::new(Lexer::new(src).peekable()).program() {
        Ok(decls) => decls,
        Err(errors) => {
            parser::print_parse_errors(errors);
            std::process::exit(0);
        }
    }).instructions.iter().for_each(|instr| println!("{:?}", instr));
   
 
}
