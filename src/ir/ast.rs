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
pub enum ExprKind {
    Binary {
        lhs: Box<ExprKind>,
        op: Token,
        rhs: Box<ExprKind>,
    },
    Logical {
        lhs: Box<ExprKind>,
        op: Token,
        rhs: Box<ExprKind>,
    },
    Unary {
        op: Token,
        expr: Box<ExprKind>,
    },
    Assignment {
        target: String,
        value: Box<ExprKind>,
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
        condition: ExprKind,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },

    Repeat {
        body: Box<Stmt>,
        until: ExprKind,
    },

    While {
        body: Box<Stmt>,
        condition: ExprKind,
    },

    Call {
        name: String,
        args: Vec<ExprKind>
    },

    VarDecl {
        name: String,
        type_name: TypeName,
    },

    Expr(ExprKind),
    Output(ExprKind),
    Input(String),
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
