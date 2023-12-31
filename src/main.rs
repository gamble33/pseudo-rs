mod args;
mod debug;

use args::Cli;
use clap::Parser;


fn main() {
    let cli = Cli::parse();
    let src = match std::fs::read_to_string(cli.source_path.clone()) {
        Ok(src) => src,
        Err(_) =>  {
            println!("Provided file path `{}` is not valid.", cli.source_path);
            std::process::exit(0);
        }
    };

    if let Some(debug_mode) = cli.debug {
        use args::DebugMode::*;
        match debug_mode {
            PrintAst => debug::print_ast(&src),
            PrintTokens => debug::print_tokens(&src),
            PrintBytecode => debug::print_bytecode(&src),
        };
        std::process::exit(0);
    };

    if cli.compile_to_c {
        pseudo_rs::compile_to_c(&src);
        std::process::exit(0);
    }

    pseudo_rs::interpret(&src);
}
