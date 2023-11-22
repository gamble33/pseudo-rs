use pseudo_rs::lexer::Lexer; 

pub fn print_tokens(src: &str) {
    Lexer::new(src).for_each(|token| println!("{:?}", token.kind));
}
