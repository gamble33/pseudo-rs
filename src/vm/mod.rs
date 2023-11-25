pub mod chunk;
pub mod instr;
pub mod value;

use crate::{
    ir::hlir::Type,
    vm::{chunk::Chunk, instr::Instr::*, value::Value},
};

pub struct Vm {
    stack: Vec<Value>,
}

impl Vm {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn execute(&mut self, chunk: Chunk) {
        macro_rules! execute_for_type {
            ($expr:expr, $type:expr, $($ident:ident),*) => {
                unsafe {
                    match $type {
                        Type::Integer => { $(let $ident = self.stack.pop().unwrap().integer;)* $expr },
                        Type::Real => { $(let $ident = self.stack.pop().unwrap().real;)* $expr },
                        Type::Char => { $(let $ident = self.stack.pop().unwrap().char;)* $expr },
                        Type::Boolean => { $(let $ident = self.stack.pop().unwrap().boolean;)* $expr },
                    }
                }
            };
        }

        macro_rules! binary_op {
            ($op:tt, $type:expr) => {
                {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    unsafe {
                        match $type {
                            Type::Integer => self.stack.push(Value {
                                integer: a.integer $op b.integer,
                            }),
                            Type::Real => self.stack.push(Value {
                                real: a.real $op b.real,
                            }),
                            Type::Char => unreachable!(),
                            Type::Boolean => unreachable!(),
                        };
                    }
                }
            };
        }

        chunk.instructions.iter().for_each(|instr| match instr {
            Const(index) => {
                let value = chunk.constants.get(*index).unwrap();
                self.stack.push(*value);
            }
            Ret => todo!(),
            Output(pseudo_type) => execute_for_type!(println!("{}", value), pseudo_type, value),
            Add(pseudo_type) => binary_op!(+, pseudo_type),
            Sub(pseudo_type) => binary_op!(-, pseudo_type),
            Mul(pseudo_type) => binary_op!(*, pseudo_type),
            Div(_pseudo_type) => unimplemented!(),
            Gt(_pseudo_type) => todo!(),
            GtEq(_pseudo_type) => todo!(),
            Eq(_pseudo_type) => todo!(),
            Neg(pseudo_type) => {
                let value = self.stack.pop().unwrap();
                unsafe {
                    match pseudo_type {
                        Type::Integer => self.stack.push(Value {
                            integer: -value.integer,
                        }),
                        Type::Real => self.stack.push(Value { real: -value.real }),
                        Type::Char => unreachable!(),
                        Type::Boolean => unreachable!(),
                    };
                };
            },
            Not => unsafe {
                let boolean = !self.stack.pop().unwrap().boolean;
                self.stack.push(Value {boolean});
            },
            True => self.stack.push(Value {boolean: true}),
            False => self.stack.push(Value {boolean: false}),
        });
    }
}
