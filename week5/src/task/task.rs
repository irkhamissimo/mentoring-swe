use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    id: u32,
    title: String,
    completed: bool,
    created_at: DateTime<Local>,
}

impl Task {
    pub fn new(id: u32, title: impl Into<String>) -> Self {
        Self {
            id,
            title: title.into(),
            completed: false,
            created_at: Local::now(),
        }
    }
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn mark_done(&mut self) -> bool {
        let was_completed = self.completed;
        self.completed = true;
        !was_completed
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.completed { "✓" } else { " " };

        write!(
            f,
            "[{}] {} (id: {}, created: {})",
            status,
            self.title,
            self.id,
            self.created_at.format("%Y-%m-%d %H:%M")
        )
    }
}
