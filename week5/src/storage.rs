use std::fmt;
use std::fs;
use std::path::PathBuf;

use crate::task::task::Task;

const FILE_PATH: &str = "tasks.json";
#[derive(Debug)]
pub enum StorageError {
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl From<std::io::Error> for StorageError {
    fn from(err: std::io::Error) -> Self {
        StorageError::Io(err)
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(err: serde_json::Error) -> Self {
        StorageError::Json(err)
    }
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::Io(err) => write!(f, "I/O error: {}", err),
            StorageError::Json(err) => write!(f, "JSON error: {}", err),
        }
    }
}
impl std::error::Error for StorageError {}
pub fn load_tasks() -> Result<Vec<Task>, StorageError> {
    let path = tasks_file_path();
    if !path.exists() {
        return Ok(Vec::new());
    }

    let contents = std::fs::read_to_string(path)?;
    let tasks = serde_json::from_str(&contents)?;
    Ok(tasks)
}
pub fn save_tasks(tasks: &Vec<Task>) -> Result<(), StorageError> {
    let path = tasks_file_path();
    let json = serde_json::to_string_pretty(tasks)?;
    std::fs::write(path, json)?;
    Ok(())
}

fn tasks_file_path() -> PathBuf {
    let mut dir = dirs::data_local_dir().expect("Could not find local data directory");
    dir.push("todolist");
    std::fs::create_dir_all(&dir).ok();

    dir.push("tasks.json");
    dir
}
