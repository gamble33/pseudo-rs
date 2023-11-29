pub mod chunk;
pub mod instr;
pub mod obj;
pub mod value;

use crate::{
    as_rs_string,
    ir::hlir::Type,
    vm::{chunk::Chunk, instr::Instr::*, obj::allocate_string, value::Value},
};

use self::obj::{free_object, Obj};

pub struct Vm {
    stack: Vec<Value>,
    objects: *mut Obj,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            objects: std::ptr::null_mut(),
        }
    }

    pub fn free_objects(&self) {
        let mut obj = self.objects;
        while !obj.is_null() {
            unsafe {
                let next = (*obj).next;
                free_object(obj);
                obj = next;
            }
        }
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

        let mut instr_idx = 0;
        while instr_idx < chunk.instructions.len() {
            match chunk.instructions[instr_idx] {
                Const(index) => {
                    let value = chunk.constants.get(index).unwrap();
                    self.stack.push(*value);
                }
                Pop => {
                    self.stack.pop();
                }
                LoadLocal(idx) => unsafe {
                    let value = self.stack.get_unchecked(idx);
                    self.stack.push(value.clone());
                },
                StoreLocal(idx) => {
                    self.stack[idx] = self.stack.last().unwrap().clone();
                }
                Ret => todo!(),
                Input => {
                    let mut input = String::new();
                    match std::io::stdin().read_line(&mut input) {
                        Ok(_) => (),
                        Err(error) => println!("error reading user input: {error}"),
                    };
                    let input = allocate_string(self, input);
                    self.stack.push(Value {obj: input});
                },
                Output(pseudo_type) => execute_for_type!(println!("{}", value), pseudo_type, value),
                Concat => unsafe {
                    let b = as_rs_string!(self.stack.pop().unwrap().obj);
                    let a = as_rs_string!(self.stack.pop().unwrap().obj);
                    let mut result = String::new();
                    result.push_str(a);
                    result.push_str(b);
                    let result = allocate_string(self, result);
                    self.stack.push(Value { obj: result });
                },
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
                    self.stack.push(Value { boolean: equality });
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
                Null => self.stack.push(Value { integer: 0 }),
                JumpFalse(idx) => unsafe {
                    if !self.stack.pop().unwrap().boolean {
                        instr_idx = idx - 1;
                    }
                },
                Jump(idx) => instr_idx = idx - 1,
            };
            instr_idx += 1;
        }
        self.free_objects(); // todo: free objects for now after executing chunk. later, change
                             // this to deallocate objects when necessary.
    }
}
