use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    completed: bool,
}

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple CLI TODO app.")]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { task: String },
    List,
    Remove,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Add { task } => {
            println!("You want to add: {}", task);
        }
        Commands::List => {
            println!("You want to list all tasks");
        }
        Commands::Remove => {
            println!("You want to remove a task.");
        }
    }
}
