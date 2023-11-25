use crate::ir::hlir::Type;

#[derive(Debug)]
pub enum Instr {
    /// Push constant to the stack with an index of the value
    Const(usize),
    Ret,
    Output(Type),
    /// Concatenates two strings
    Concat,
    Add(Type),
    Sub(Type),
    Mul(Type),
    Div(Type),
    Gt(Type),
    GtEq(Type),
    Eq(Type),
    Neg(Type),
    Not,
    True,
    False,
}
