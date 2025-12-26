use std::fmt;
use std::fs;
use std::path::Path;

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
    if !Path::new(FILE_PATH).exists() {
        return Ok(Vec::new());
    }

    let contents = fs::read_to_string(FILE_PATH)?;
    let tasks = serde_json::from_str(&contents)?;
    Ok(tasks)
}
pub fn save_tasks(tasks: &Vec<Task>) -> Result<(), StorageError> {
    let json = serde_json::to_string_pretty(tasks)?;
    fs::write(FILE_PATH, json)?;
    Ok(())
}
