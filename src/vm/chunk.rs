use crate::vm::instruction::OpCode;

pub struct Chunk {
    op_codes: Vec<OpCode>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            op_codes: Vec::new(),
        }
    }
}
