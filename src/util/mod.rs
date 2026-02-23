use clap::{Parser, Subcommand};
use crate::Notes;

pub mod trav;
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
    Delete{index:usize},
    Complete{index:usize},
    GetCompleted{}
}

pub fn increase_index(todos:&Vec<Notes>) -> i32 {
    return todos.last().map_or(1, |t| t.index + 1);
}


