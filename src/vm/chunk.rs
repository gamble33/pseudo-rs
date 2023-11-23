use crate::vm::instr::Instr;
use super::value::Value;

pub struct Chunk {
    pub instructions: Vec<Instr>,
    pub constants: Vec<Value>
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);
        self.constants.len() - 1
    }
}
