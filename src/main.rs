use pseudo_rs::{compile_to_c, interpret};

fn main() {

    match std::env::args().nth(1) {
        Some(path) => {
            match std::fs::read_to_string(path.clone()) {
                Ok(src) => compile_to_c(&src),
                Err(_) =>  {
                    println!("Provided file path `{}` is not valid.", path);
                    std::process::exit(0);
                }
            }
        }
        None => {
            println!("You must provide a file path.");
            println!("Usage: pseudo <file path>");
            std::process::exit(0);
        }
    }
}
