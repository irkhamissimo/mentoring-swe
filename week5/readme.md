# 📝 Rust To-Do CLI

A simple command-line **to-do list application** written in Rust.  
Tasks are stored locally in a JSON file and can be added, listed, marked as done, and removed.

This project is built as a learning exercise to understand:
- Rust module structure
- Ownership & borrowing
- Error handling
- CLI argument parsing with `clap`
- File-based persistence with `serde_json`

---

## ✨ Features

- ✅ Add new tasks
- 📋 List all tasks
- ✔️ Mark tasks as done
- 🗑 Remove tasks
- 💾 Persistent storage using `tasks.json`
- 🕒 Timestamp for task creation

---

## 📦 Requirements

- Rust (stable)
- Cargo

## Run the application using cargo run:

Add a task
cargo run -- add "Learn Rust"

List tasks
cargo run -- list


## Example output:

[ ] Learn Rust (id: 1, created: 2025-01-01 19:30)
[✓] Write README (id: 2, created: 2025-01-01 20:00)

Mark a task as done
cargo run -- done 1

Remove a task
cargo run -- remove 2


file is located in: ~/.local/share/todolist/tasks.json
