use pseudo_rs::{codegen_bytecode::emit, lexer::Lexer, naive_tc, parser, parser::Parser, vm::Vm};

pub fn print_bytecode(src: &str) {
    emit(
        naive_tc::typecheck(match Parser::new(Lexer::new(src).peekable()).program() {
            Ok(decls) => decls,
            Err(errors) => {
                parser::print_parse_errors(errors);
                std::process::exit(0);
            }
        }),
        &mut Vm::new(),
    )
    .chunk
    .instructions
    .iter()
    .enumerate()
    .for_each(|(idx, instr)| println!("{idx}\t{:?}", instr));
}
