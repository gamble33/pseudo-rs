pub mod chunk;
pub mod instr;
pub mod value;
pub mod obj;

use crate::{
    as_rs_string,
    ir::hlir::Type,
    vm::{chunk::Chunk, instr::Instr::*, value::Value, obj::allocate_string},
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
                        Type::String => { $(let $ident = as_rs_string!(self.stack.pop().unwrap().obj);)* $expr },
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
                            _ => unreachable!()
                        };
                    }
                }
            };
        }

        macro_rules! binary_comparison {
            ($op:tt, $type:expr) => {
                {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    unsafe {
                        match $type {
                            Type::Integer => self.stack.push(Value {
                                boolean: a.integer $op b.integer,
                            }),
                            Type::Real => self.stack.push(Value {
                                boolean: a.real $op b.real,
                            }),
                            Type::Char => unimplemented!("Do we want comparison of CHARs?"),
                            _ => unreachable!(),
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
            Concat => unsafe {
                let b = as_rs_string!(self.stack.pop().unwrap().obj);
                let a = as_rs_string!(self.stack.pop().unwrap().obj);
                let mut result = String::new();
                result.push_str(a);
                result.push_str(b);
                self.stack.push(Value { obj: allocate_string(result) });
            }
            Add(pseudo_type) => binary_op!(+, pseudo_type),
            Sub(pseudo_type) => binary_op!(-, pseudo_type),
            Mul(pseudo_type) => binary_op!(*, pseudo_type),
            Div(_pseudo_type) => unimplemented!(),
            Gt(pseudo_type) => binary_comparison!(>, pseudo_type),
            GtEq(pseudo_type) => binary_comparison!(>=, pseudo_type),
            Eq(pseudo_type) => unsafe {
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                let equality = match pseudo_type {
                    Type::Integer => a.integer == b.integer,
                    Type::Real => a.real == b.real,
                    Type::Char => a.char == b.char,
                    Type::Boolean => a.boolean == b.boolean,
                    Type::String => todo!(),
                };
                self.stack.push(Value {boolean: equality});
            },
            Neg(pseudo_type) => {
                let value = self.stack.pop().unwrap();
                unsafe {
                    match pseudo_type {
                        Type::Integer => self.stack.push(Value {
                            integer: -value.integer,
                        }),
                        Type::Real => self.stack.push(Value { real: -value.real }),
                        _ => unreachable!(),
                    };
                };
            }
            Not => unsafe {
                let boolean = !self.stack.pop().unwrap().boolean;
                self.stack.push(Value { boolean });
            },
            True => self.stack.push(Value { boolean: true }),
            False => self.stack.push(Value { boolean: false }),
        });
    }
}
