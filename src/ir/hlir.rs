use super::ast;
use crate::lexer::token::Token;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Type {
    Integer,
    Real,
    Char,
    Boolean,
    String,
}

#[derive(Debug, Clone)]
pub struct Expr {
    pub pseudo_type: Type,
    pub expr_kind: ExprKind,
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Binary {
        lhs: Box<Expr>,
        op: Token,
        rhs: Box<Expr>,
    },
    Logical {
        lhs: Box<Expr>,
        op: Token,
        rhs: Box<Expr>,
    },
    Unary {
        op: Token,
        expr: Box<Expr>,
    },
    Assignment {
        target: String,
        value: Box<Expr>,
    },
    Call {
        callee: String,
        args: Vec<Expr>
    },
    Literal(ast::LiteralKind),
    Variable(String),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },

    Repeat {
        body: Box<Stmt>,
        until: Expr,
    },

    While {
        body: Box<Stmt>,
        condition: Expr,
    },

    Call {
        name: String,
        args: Vec<Expr>
    },

    VarDecl {
        name: String,
    },

    Return(Expr),
    Expr(Expr),
    Output(Vec<Expr>),
    Input(String),
    Block(Vec<Stmt>),
}

#[derive(Debug, Clone)]
pub enum Decl {
    Procedure {
        name: String,
        params: Vec<Param>,
        body: Stmt,
    },
    Function {
        name: String,
        params: Vec<Param>,
        body: Stmt,
        return_type: Type,
    }
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub pseudo_type: Type,
    pub passing_mode: Option<ast::PassingMode>,
}
