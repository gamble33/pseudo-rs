use pseudo_rs::{
    codegen_bytecode::emit,
    lexer::Lexer,
    naive_tc,
    parser::program,
    vm::Vm,
    error
};

pub fn print_bytecode(src: &str) {
    emit(
        naive_tc::typecheck(match program(Lexer::new(src).peekable()) {
            Ok(decls) => decls,
            Err(errors) => {
                error::print_parse_errors(src, errors);
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
