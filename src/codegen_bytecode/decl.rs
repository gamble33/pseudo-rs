use crate::{
    codegen_bytecode::Generator,
    ir::hlir::{Decl, Param},
    vm::{
        chunk::Chunk,
        instr::Instr,
        obj::{allocate_string, store_function, Obj, ObjFn, ObjKind},
        value::Value,
    },
};

impl Generator<'_> {
    pub fn define_decl(&mut self, decl: &Decl) {
        match decl {
            Decl::Procedure { name, params, body } => {
                self.emit(Instr::Null);
                self.add_local(name.clone());
            }
        }
    }

    pub fn decl(&mut self, decl: &Decl) {
        match decl {
            Decl::Procedure { name, params, body } => {
                let function = ObjFn {
                    obj: Obj {
                        kind: ObjKind::Fn,
                        next: std::ptr::null_mut(),
                    },
                    chunk: Chunk::new(),
                    name: allocate_string(self.vm, name.to_string()),
                };
                let previous_function = std::mem::replace(&mut self.function, Some(function));
                self.params(params);
                self.stmt(body);
                self.emit(Instr::Ret);
                let function = std::mem::replace(&mut self.function, previous_function);
                let function = store_function(self.vm, function.unwrap());

                let procedure_idx = self.resolve_local(name);
                self.emit_constant(Value { obj: function });
                self.emit(Instr::StoreLocal(procedure_idx));
            }
        }
    }

    fn params(&mut self, params: &Vec<Param>) {
        params.iter().for_each(|param| {
            self.emit(Instr::Null);
            self.add_local(param.name.clone());
        })
    }
}
