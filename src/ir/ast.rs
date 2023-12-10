use crate::lexer::token::Token;

#[derive(Debug, Clone)]
pub enum TypeName {
    BaseTypeName(BaseTypeName),
}

#[derive(Debug, Clone)]
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
    Call {
        callee: Box<ExprKind>,
        args: Vec<ExprKind>,
    },
    Literal(LiteralKind),
    Variable(String),
}

#[derive(Debug, Clone)]
pub enum LiteralKind {
    Integer(i64),
    Real(f64),
    Character(char),
    String(String),
    Boolean(bool),
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
        return_type_name: TypeName,
    }
}

#[derive(Debug, Clone)]
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

    Return(ExprKind),
    Expr(ExprKind),
    Output(ExprKind),
    Input(String),
    Block(Vec<Stmt>),
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub type_name: TypeName,
    pub passing_mode: Option<PassingMode>,

}

#[derive(Debug, Copy, Clone)]
pub enum PassingMode {
    ByVal,
    ByRef,
}
