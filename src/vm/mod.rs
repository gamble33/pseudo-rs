mod value;
pub mod chunk;
mod instruction;

use crate::vm::value::Value;

pub struct Vm {
    stack: Vec<Value>
}

impl Vm {
    pub fn new() -> Self {
        Self {
            stack: Vec::new()
        }
    }

    pub fn run() {

    }
}
