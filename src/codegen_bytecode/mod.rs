#[allow(unused_variables)] mod decl;
#[allow(unused_variables)] mod expr;
#[allow(unused_variables)] mod stmt;

use crate::ir::hlir::Decl;
use crate::vm::chunk::Chunk;

struct Generator {
    target: Chunk,
}

pub fn emit(program: Vec<Decl>) -> Chunk {
    let mut generator = Generator { target: Chunk::new() };
    generator.decl(&program[0]);
    generator.target
}
