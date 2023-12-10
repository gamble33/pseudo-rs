pub mod chunk;
pub mod instr;
pub mod obj;
pub mod value;

use self::{
    chunk::Chunk,
    obj::{free_object, Obj, ObjFn},
};
use crate::{
    as_rs_string,
    ir::hlir::Type,
    vm::{
        instr::Instr::*,
        obj::{allocate_string, store_function},
        value::Value,
    },
};

struct CallFrame {
    function: *mut ObjFn,
    ret_instr_idx: usize,
    window_start_idx: usize,
}

pub struct Vm {
    stack: Vec<Value>,
    frames: Vec<CallFrame>,
    objects: *mut Obj,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            frames: Vec::new(),
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

    pub fn execute(&mut self, script: ObjFn) {
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

        let script = store_function(self, script);
        self.frames.push(CallFrame {
            function: script as *mut ObjFn,
            ret_instr_idx: 0,
            window_start_idx: 0,
        });

        let mut instr_idx = 0;
        while instr_idx < self.get_current_chunk().instructions.len() {
            let mut instr_inc = 1;
            match self.get_current_chunk().instructions[instr_idx] {
                Const(index) => {
                    let value = self.get_current_chunk().constants.get(index).unwrap();
                    self.stack.push(*value);
                }
                Pop => {
                    self.stack.pop();
                }
                LoadLocal(idx) => unsafe {
                    let window_start_idx = self.frames.last().unwrap().window_start_idx;
                    let value = self.stack.get_unchecked(window_start_idx + idx);
                    self.stack.push(value.clone());
                },
                StoreLocal(idx) => {
                    let window_start_idx = self.frames.last().unwrap().window_start_idx;
                    self.stack[window_start_idx + idx] = self.stack.last().unwrap().clone();
                }
                LoadGlobal(idx) => unsafe {
                    let value = (*(script as *mut ObjFn)).chunk.constants.get_unchecked(idx);
                    self.stack.push(value.clone());
                },
                StoreGlobal(idx) => unsafe {
                    let value = self.stack.last().unwrap();
                    (*(script as *mut ObjFn)).chunk.constants[idx] = *value;
                },
                Call(args_amount) => unsafe {
                    let function = self
                        .stack
                        .get_unchecked(self.stack.len() - args_amount - 1)
                        .obj as *mut ObjFn;

                    // println!(
                    //     "{}: {:?}",
                    //     as_rs_string!((*function).name),
                    //     (*function).chunk.instructions
                    // );

                    // self.stack.iter().for_each(|value| println!("{}", value.integer));

                    self.frames.push(CallFrame {
                        function,
                        ret_instr_idx: instr_idx,
                        window_start_idx: self.stack.len() - args_amount,
                    });
                    instr_idx = 0;
                    instr_inc = 0;
                },
                Ret(args_amount) => {
                    let return_value = self.stack.pop().unwrap();

                    // pop args and function reference off stack.
                    for _ in 0..(args_amount + 1) {
                        self.stack.pop();
                    }

                    let call_frame = self.frames.pop().unwrap();
                    instr_idx = call_frame.ret_instr_idx;
                    self.stack.push(return_value);
                }
                Input => {
                    let mut input = String::new();
                    match std::io::stdin().read_line(&mut input) {
                        Ok(_) => (),
                        Err(error) => println!("error reading user input: {error}"),
                    };
                    input = String::from(input.trim_end());
                    let input = allocate_string(self, input);
                    self.stack.push(Value { obj: input });
                }
                Output(pseudo_type) => unsafe {
                    let value = self.stack.pop().unwrap();
                    match pseudo_type {
                        Type::Integer => println!("{}", value.integer),
                        Type::Real => println!("{}", value.real),
                        Type::Char => println!("{}", value.char),
                        Type::Boolean => {
                            println!("{}", if value.boolean { "TRUE" } else { "FALSE" })
                        }
                        Type::String => println!("{}", as_rs_string!(value.obj)),
                    }
                },
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
                Div(pseudo_type) => unsafe {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match pseudo_type {
                        Type::Integer => self.stack.push(Value {integer: a.integer / b.integer}),
                        Type::Real => self.stack.push(Value {real: a.real / b.real}),
                        _ => unreachable!(),
                    };
                },
                Mod(pseudo_type) => unsafe {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match pseudo_type {
                        Type::Integer => self.stack.push(Value {integer: a.integer % b.integer}),
                        Type::Real => self.stack.push(Value {real: a.real % b.real}),
                        _ => unreachable!(),
                    };
                },
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
                        Type::String => {
                            let a = as_rs_string!(a.obj);
                            let b = as_rs_string!(b.obj);
                            a == b
                        }
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
                    if !self.stack.last().unwrap().boolean {
                        instr_idx = idx - 1;
                    }
                },
                JumpTrue(idx) => unsafe {
                    if self.stack.last().unwrap().boolean {
                        instr_idx = idx - 1;
                    }
                },
                Jump(idx) => instr_idx = idx - 1,
            };
            instr_idx += instr_inc;
        }
        self.free_objects(); // todo: free objects for now after executing chunk. later, change
                             // this to deallocate objects when necessary.
    }

    fn get_current_chunk(&self) -> &Chunk {
        unsafe { &(*self.frames.last().unwrap().function).chunk }
    }
}
