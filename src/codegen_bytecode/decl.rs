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
            Decl::Procedure { name, .. } => {
                self.declare_global(name.clone());
            }
            Decl::Function { name, .. } => {
                self.declare_global(name.clone());
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
                let previous_locals = std::mem::replace(&mut self.locals, Vec::new());

                self.params(params);
                self.stmt(body);
                self.emit(Instr::Null);
                self.emit(Instr::Ret(params.len()));

                let function = std::mem::replace(&mut self.function, previous_function);
                self.locals = previous_locals;

                let function = store_function(self.vm, function.unwrap());

                self.emit_constant(Value { obj: function });
                self.emit(Instr::StoreGlobal(self.resolve_global(name)));
                self.emit(Instr::Pop);
            }
            Decl::Function {
                name,
                params,
                body,
                ..
            } => {
                let function = ObjFn {
                    obj: Obj {
                        kind: ObjKind::Fn,
                        next: std::ptr::null_mut(),
                    },
                    chunk: Chunk::new(),
                    name: allocate_string(self.vm, name.to_string()),
                };

                let previous_function = std::mem::replace(&mut self.function, Some(function));
                let previous_locals = std::mem::replace(&mut self.locals, Vec::new());
                let previous_args_len = std::mem::replace(&mut self.current_function_args, Some(params.len()));

                self.params(params);
                self.stmt(body);

                let function = std::mem::replace(&mut self.function, previous_function);
                self.locals = previous_locals;
                self.current_function_args = previous_args_len;

                let function = store_function(self.vm, function.unwrap());

                self.emit_constant(Value { obj: function });
                self.emit(Instr::StoreGlobal(self.resolve_global(name)));
                self.emit(Instr::Pop);
            }
        }
    }

    fn params(&mut self, params: &Vec<Param>) {
        params.iter().for_each(|param| {
            self.add_local(param.name.clone());
        })
    }
}
