use pseudo_rs::{
    ir::ast::{BaseTypeName, ExprKind, Stmt, TypeName, Decl, LiteralKind},
    lexer::{token::Token, token::TokenKind, Lexer},
    parser::program,
    error,
};

pub fn print_ast(src: &str) {
    let program = program(Lexer::new(src).peekable());
    match program {
        Ok(decls) => decls.iter().for_each(|decl| print_decl(decl)),
        Err(errors) => {
            error::print_parse_errors(src, errors);
            std::process::exit(0);
        }
    };
}

fn print_decl(decl: &Decl) {
    match decl {
        Decl::Procedure { name, params, body } => {
            println!("proc {} ({:?})", name, params);
            print_stmt(body, 1);
        }
        Decl::Function { name, params, body, return_type_name } => {
            println!("fn {} ({:?}) -> {:?}", name, params, return_type_name);
            print_stmt(body, 1);
        }
    }
}

fn print_stmt(stmt: &Stmt, depth: u32) {
    print_depth(depth);
    match stmt {
        Stmt::If {
            condition,
            then_branch,
            else_branch,
        } => {
            println!("if");
            print_expr(condition, depth + 1);
            print_stmt(then_branch, depth + 1);
            if let Some(else_branch) = else_branch {
                print_stmt(else_branch, depth + 1);
            }
        }

        Stmt::Call { name, args } => {
            println!("call {}", name);
            args.iter()
                .enumerate()
                .for_each(|(_index, arg)| print_expr(arg, depth + 1));
        }
        Stmt::Block(stmts) => {
            println!("block");
            stmts.iter().for_each(|stmt| print_stmt(stmt, depth + 1));
        }
        Stmt::While { body, condition } => {
            println!("while");
            print_expr(condition, depth + 1);
            print_stmt(body, depth + 1);
        }
        Stmt::Repeat { body, until } => {
            println!("repeat");
            print_expr(until, depth + 1);
            print_stmt(body, depth + 1);
        }
        Stmt::VarDecl { name, type_name } => {
            print!("var {}: ", name);
            print_type_name(type_name);
            println!();
        }
        Stmt::Return(expr) => {
            println!("return");
            print_expr(expr, depth + 1);

        }
        Stmt::Input(target) => {
            println!("input");
            print_depth(depth + 1);
            print!("{}", target);
        }
        Stmt::Expr(expr) => {
            println!("expr stmt");
            print_expr(expr, depth + 1);
        }
        Stmt::Output(exprs) => {
            println!("output");
            exprs.iter().for_each(|expr| {
                print_expr(expr, depth + 1);
            });
        }
    }
}

fn print_expr(expr: &ExprKind, depth: u32) {
    print_depth(depth);
    match expr {
        ExprKind::Unary { op, expr } => {
            print_operator(op);
            print_expr(expr, depth + 1);
        }
        ExprKind::Binary { lhs, op, rhs } => {
            print_operator(op);
            print_expr(lhs, depth + 1);
            print_expr(rhs, depth + 1);
        }
        ExprKind::Logical { lhs, op, rhs } => {
            print_operator(op);
            print_expr(lhs, depth + 1);
            print_expr(rhs, depth + 1);
        }
        ExprKind::Assignment { target, value } => {
            println!("x <- y");
            print_depth(depth);
            print!("{}", target);
            print_expr(value, depth + 1);
        }
        ExprKind::Call { callee, args } => {
            println!("fn call");

            print_depth(depth + 1);
            println!("callee:");
            print_expr(callee, depth + 2);

            print_depth(depth);
            println!("args:");
            args.iter().for_each(|arg| print_expr(arg, depth + 2));
        }
        ExprKind::Variable(name) => {
            println!("var {}", name);
        }
        ExprKind::Literal(literal) => match literal {
            LiteralKind::Integer(i) => println!("{}", i),
            LiteralKind::Real(f) => println!("{}", f),
            LiteralKind::Character(ch) => println!("'{}'", ch),
            LiteralKind::String(string) => println!("\"{}\"", string),
            LiteralKind::Boolean(b) => println!("{}", b),
        },
    }
}

fn print_depth(depth: u32) {
    for _ in 0..depth {
        print!("-- ");
    }
}

fn print_type_name(type_name: &TypeName) {
    match type_name {
        TypeName::BaseTypeName(base_type_name) => match base_type_name {
            BaseTypeName::Integer => print!("int"),
            BaseTypeName::Real => print!("real"),
            BaseTypeName::String => print!("str"),
            BaseTypeName::Char => print!("ch"),
            BaseTypeName::Boolean => print!("bool"),
            BaseTypeName::Date => print!("date"),
            BaseTypeName::Identifier(_name) => todo!(),
        },
    }
}

fn print_operator(op: &Token) {
    use TokenKind::*;
    println!(
        "{}",
        match &op.kind {
            Keyword(keyword) => match keyword {
                pseudo_rs::lexer::token::KeywordKind::Or => "or",
                pseudo_rs::lexer::token::KeywordKind::And => "and",
                pseudo_rs::lexer::token::KeywordKind::Not => "not",
                _ => unreachable!(),
            },

            Greater => ">",
            GreaterEqual => ">=",
            Less => "<",
            LessEqual => "<=",

            Equal => "=",
            NotEqual => "<>",

            Plus => "+",
            Minus => "-",
            Star => "*",
            Slash => "/",
            Ampersand => "&",
            _ => unreachable!(),
        }
    );
}
