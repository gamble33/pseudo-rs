#[derive(Debug)]
pub enum Instr {
    /// Push constant to the stack with an index of the value
    Const(usize),
    Ret,
    Output,
    Add,
    Sub,
    Mul,
    Div,
    Gt,
    GtEq,
    Eq,
    Neg,
    Not,
}
