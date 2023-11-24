use crate::lexer::token::Token;

#[derive(Debug)]
pub enum TypeName {
    BaseTypeName(BaseTypeName),
}

#[derive(Debug)]
pub enum BaseTypeName {
    Integer,
    Real,
    String,
    Char,
    Boolean,
    Date,
    Identifier(String),
}

#[derive(Debug, Clone)]
pub enum Expr {
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
        target: Box<Expr>,
        value: Box<Expr>,
    },
    Literal(LiteralKind),
    Variable(String),
}

#[derive(Debug, Clone)]
pub enum LiteralKind {
    Integer(i32),
    Character(char),
    String(String),
    Boolean(bool),
}


#[derive(Debug)]
pub enum Decl {
    Procedure {
        name: String,
        params: Vec<Param>,
        body: Stmt,
    },
}

#[derive(Debug)]
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
        type_name: TypeName,
    },

    Expr(Expr),
    Output(Expr),
    Input(Expr),
    Block(Vec<Stmt>),
}

#[derive(Debug)]
pub struct Param {
    pub name: String,
    pub type_name: TypeName,
    pub passing_mode: Option<PassingMode>,
}

#[derive(Debug)]
pub enum PassingMode {
    ByVal,
    ByRef,
}
