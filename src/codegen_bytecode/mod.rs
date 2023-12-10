mod decl;
mod expr;
mod stmt;
mod local;
mod global;

use std::collections::HashMap;

use self::local::Local;
use crate::ir::hlir::Decl;
use crate::vm::Vm;
use crate::vm::chunk::Chunk;
use crate::vm::instr::Instr;
use crate::vm::obj::{ObjFn, Obj, ObjKind};
use crate::vm::value::Value;

struct Generator<'a> {
    function: Option<ObjFn>,
    vm: &'a mut Vm,
    scope_depth: u8,
    locals: Vec<Local>,
    globals: HashMap<String, usize>,
}

pub fn emit<'a>(program: Vec<Decl>, vm: &'a mut Vm) -> ObjFn {
    let script = ObjFn {
        obj: Obj { kind: ObjKind::Fn, next: std::ptr::null_mut() },
        chunk: Chunk::new(),
        name: std::ptr::null_mut(), // todo: add a name to top-level function.
    };

    let mut generator = Generator {
        function: Some(script),
        vm,
        scope_depth: 0,
        locals: Vec::new(),
        globals: HashMap::new(),
    };

    // decalre each declaration
    program.iter().for_each(|decl| generator.define_decl(decl));     

    // emit bytecode for each declaration
    program.iter().for_each(|decl| generator.decl(decl)); 

    // Call main procedure at top-level script.
    let main_procedure = generator.resolve_global("Main");
    generator.emit(Instr::LoadGlobal(main_procedure));
    generator.emit(Instr::Call(0)); // todo: allow args passed to main proc

    let script = generator.function.unwrap();
    script
}

impl Generator<'_> {
    fn target(&mut self) -> &mut Chunk {
        &mut self.function
            .as_mut()
            .expect("Attempted access of `None` chunk.")
            .chunk
    }

    fn emit(&mut self, instr: Instr) {
        self.target().instructions.push(instr);
    }

    fn emit_constant(&mut self, value: Value) {
        let instr = Instr::Const(self.target().add_constant(value));
        self.emit(instr);
    }
}
