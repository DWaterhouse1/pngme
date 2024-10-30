use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    Encode {
        path: PathBuf,
        chunk_type: String,
        message: String,
        output: Option<PathBuf>,
    },

    #[command(arg_required_else_help = true)]
    Decode { path: PathBuf, chunk_type: String },

    #[command(arg_required_else_help = true)]
    Remove { path: PathBuf, chunk_type: String },

    #[command(arg_required_else_help = true)]
    Print { path: PathBuf },
}
