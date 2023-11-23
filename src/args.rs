use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "pseudo_rs", author = "slopey", version)]
#[command(about = "Compiler and VM for Cambridge's A-Level pseudocode specification")]
pub struct Cli {
    /// Debugging options such as printing the AST or dumping the tokens
    #[arg(short, long, value_enum)]
    pub debug: Option<DebugMode>,
    /// Compile the source file to C instead of interpreting it
    #[arg(long)]
    pub compile_to_c: bool,
    /// Path to the pseudocode source file
    pub source_path: String,
}

#[derive(ValueEnum, Clone)]
pub enum DebugMode {
    PrintTokens,
    PrintAst,
    PrintBytecode,
}

