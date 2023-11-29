#[allow(unused_variables)] mod decl;
#[allow(unused_variables)] mod expr;
#[allow(unused_variables)] mod stmt;
mod local;

use self::local::Local;
use crate::ir::hlir::Decl;
use crate::vm::Vm;
use crate::vm::chunk::Chunk;

struct Generator<'a> {
    target: Chunk,
    vm: &'a mut Vm,
    scope_depth: u8,
    locals: Vec<Local>

}

pub fn emit<'a>(program: Vec<Decl>, vm: &'a mut Vm) -> Chunk {
    let mut generator = Generator {
        target: Chunk::new(), 
        vm,
        scope_depth: 0,
        locals: Vec::new(),
    };
    generator.decl(&program[0]);
    generator.target
}
