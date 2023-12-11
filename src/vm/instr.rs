use crate::ir::hlir::Type;

#[derive(Debug)]
pub enum Instr {
    /// Push constant to the stack with an index of the value
    Const(usize),
    Pop,
    LoadLocal(usize),
    StoreLocal(usize),
    LoadGlobal(usize),
    StoreGlobal(usize),
    Call(usize),
    Ret(usize),
    Input,
    Output(Type),
    OutputLn,
    OutputSpace,
    /// Concatenates two strings
    Concat,
    Add(Type),
    Sub(Type),
    Mul(Type),
    Div(Type),
    Mod(Type),
    Gt(Type),
    GtEq(Type),
    Eq(Type),
    Neg(Type),
    Not,
    True,
    False,
    Null,
    JumpFalse(usize),
    JumpTrue(usize),
    Jump(usize),
}
