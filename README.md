# todo-rs

A blazingly fast CLI todo app written in Rust with SQLite backend and due date tracking.

## Features

- Fast: written in Rust with SQLite backend
- Add, list, complete, delete tasks
- Due dates with overdue highlighting
- Priority levels (low, medium, high, critical)
- Tags and filtering
- Markdown export

## Quick Start

```bash
cargo install todo-rs
todo add "Buy groceries" --due tomorrow --priority high
todo list
todo done 1
```
