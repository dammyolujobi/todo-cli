use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "notes")]
#[command(about = " A simple CLI notes tool", long_about = None)]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add { note: String },
    List {},
    Clear {},
}
