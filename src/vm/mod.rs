pub mod value;
pub mod chunk;
pub mod instr;

use crate::vm::{value::Value, chunk::Chunk, instr::Instr::*};

pub struct Vm {
    stack: Vec<Value>
}

impl Vm {
    pub fn new() -> Self {
        Self {
            stack: Vec::new()
        }
    }

    pub fn execute(&mut self, chunk: Chunk) {
        chunk.instructions.iter().for_each(|instr| match instr {
            Const(index) => {
                let value = chunk.constants.get(*index).unwrap();
                self.stack.push(*value);
            },
            Ret => todo!(),
            Output => {
                println!("{}", self.stack.pop().unwrap());
            },
            Add => {
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a + b);
            },
            Sub => {
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a - b);
            }, 
            Mul => {
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                self.stack.push(a * b);
            },
            Div => unimplemented!(),
            Gt => todo!(),
            GtEq => todo!(),
            Eq => todo!(),
            Neg => {
                let value = self.stack.pop().unwrap();
                self.stack.push(-value);
            },
            Not => todo!(),

        });
    }
}
