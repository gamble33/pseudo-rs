use pseudo_rs::{
    lexer::{Lexer, token::Token, token::TokenKind},
    parser,
    parser::{
        Parser,
        stmt::Stmt,
        expr::Expr,
        type_name::{
            BaseTypeName,
            TypeName
        }
    },
    parser::stmt::Decl
};

pub fn print_ast(src: &str) {
    let program = Parser::new(Lexer::new(src).peekable()).program();
    match program {
        Ok(decls) => decls.iter().for_each(|decl| print_decl(decl)),
        Err(errors) => {
            parser::print_parse_errors(errors);
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
    }
}

fn print_stmt(stmt: &Stmt, depth: u32) {
    print_depth(depth);
    match stmt {
        Stmt::If { condition, then_branch, else_branch } => {
            println!("if");
            print_expr(condition, depth + 1);
            print_stmt(then_branch, depth + 1);
            if let Some(else_branch) = else_branch {
                print_stmt(else_branch, depth + 1);
            }
        },
        
        Stmt::Call { name, args } => {
            println!("call {}", name);
            args.iter().enumerate().for_each(|(_index, arg)| print_expr(arg, depth + 1));
        },
        Stmt::Block(stmts) => {
            println!("block");
            stmts.iter().for_each(|stmt| print_stmt(stmt, depth + 1));
        },
        Stmt::While { body, condition } => {
            println!("while");
            print_expr(condition, depth + 1);
            print_stmt(body, depth + 1);
        },
        Stmt::Repeat { body, until } => {
            println!("repeat");
            print_expr(until, depth + 1);
            print_stmt(body, depth + 1);
        },
        Stmt::VarDecl { name, type_name } => {
            print!("var {}: ", name);
            print_type_name(type_name);
            println!();
        }
        Stmt::Input(expr) => {
            println!("input");
            print_expr(expr, depth + 1);
        },
        Stmt::Expr(expr) => {
            println!("expr stmt");
            print_expr(expr, depth + 1);
        },
        Stmt::Output(expr) => {
            println!("output");
            print_expr(expr, depth + 1);
        },
    }
}

fn print_expr(expr: &Expr, depth: u32) {
    print_depth(depth);
    match expr {
        Expr::Unary { op, expr } => {
            print_operator(op);
            print_expr(expr, depth + 1); 
        },
        Expr::Binary { lhs, op, rhs } => {
            print_operator(op);
            print_expr(lhs, depth + 1); 
            print_expr(rhs, depth + 1); 
        },
        Expr::Logical { lhs, op, rhs } => {
            print_operator(op);
            print_expr(lhs, depth + 1); 
            print_expr(rhs, depth + 1); 
        },
        Expr::Assignment { target, value } => {
            println!("x <- y");
            print_expr(target, depth + 1); 
            print_expr(value, depth + 1); 
        }
        Expr::Variable(name) => {
            println!("var {}", name);
        },
        Expr::Literal(literal) => match literal {
          parser::expr::LiteralKind::Integer(i) => println!("{}", i),
          parser::expr::LiteralKind::Character(ch) => println!("'{}'", ch),
          parser::expr::LiteralKind::String(string) => println!("\"{}\"", string),
          parser::expr::LiteralKind::Boolean(b) => println!("{}", b),
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
        }
    }
}

fn print_operator(op: &Token) {
    use TokenKind::*;
    println!("{}", match &op.kind {
        Keyword(keyword) => match keyword {
            pseudo_rs::lexer::token::KeywordKind::Or => "or",
            pseudo_rs::lexer::token::KeywordKind::And => "and",
            _ => unreachable!()
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
        _ => unreachable!()
    });
}
