use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "pseudo_rs", author = "slopey", version)]
#[command(about = "Compiler and VM for Cambridge's A-Level pseudocode specification")]
pub struct Cli {
    #[arg(short, long, value_enum)]
    pub debug: Option<DebugMode>,

    /// Path to the pseudocode source file
    pub source_path: String,
}

#[derive(ValueEnum, Clone)]
pub enum DebugMode {
    PrintTokens,
    PrintAst,
}

