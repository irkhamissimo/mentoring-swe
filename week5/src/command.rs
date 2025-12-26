use crate::storage;
use crate::task::Task;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "Simple Rust To-Do CLI")]

pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Add { title: String },
    List,
    Done { id: u32 },
    // Remove { id: u32 },
}
pub fn handle_add(title: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = storage::load_tasks()?;

    let next_id = tasks.iter().map(|t| t.id()).max().unwrap_or(0) + 1;
    let task = Task::new(next_id, title);

    tasks.push(task);
    storage::save_tasks(&tasks)?;

    println!("Task added");

    Ok(())
}

pub fn handle_list() -> Result<(), Box<dyn std::error::Error>> {
    let tasks = storage::load_tasks()?;

    if tasks.is_empty() {
        println!("No tasks yet 🎉");
        return Ok(());
    }

    for task in tasks {
        println!("{}", task);
    }

    Ok(())
}

pub fn handle_done(id: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut tasks = storage::load_tasks()?;

    let mut found = false;

    for task in tasks.iter_mut() {
        if task.id() == id {
            task.mark_done();
            found = true;
            break;
        }
    }

    if !found {
        println!("Task with id {} not found", id);
        return Ok(());
    }

    storage::save_tasks(&tasks)?;
    println!("Task {} marked as done ✓", id);

    Ok(())
}
