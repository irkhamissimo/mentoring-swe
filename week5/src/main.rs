mod command;
mod storage;
mod task;

use clap::Parser;
use command::{Cli, Commands, handle_add, handle_done, handle_list, handle_remove};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { title } => {
            handle_add(title)?;
        }
        Commands::List => {
            handle_list()?;
        }
        Commands::Done { id } => handle_done(id)?,
        Commands::Remove { id } => handle_remove(id)?,
    }

    Ok(())
}
