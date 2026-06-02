use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

const DB_FILE: &str = "todos.json";

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Todo {
    id: u64,
    title: String,
    done: bool,
    due: Option<String>,
}

struct TodoApp {
    todos: Vec<Todo>,
    next_id: u64,
    path: PathBuf,
}

impl TodoApp {
    fn load() -> Self {
        let path = PathBuf::from(DB_FILE);
        if path.exists() {
            let data = fs::read_to_string(&path).unwrap_or_default();
            let todos: Vec<Todo> = serde_json::from_str(&data).unwrap_or_default();
            let next_id = todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            TodoApp { todos, next_id, path }
        } else {
            TodoApp { todos: vec![], next_id: 1, path }
        }
    }

    fn save(&self) {
        let data = serde_json::to_string_pretty(&self.todos).unwrap();
        fs::write(&self.path, data).unwrap();
    }

    fn add(&mut self, title: String, due: Option<String>) {
        let todo = Todo { id: self.next_id, title, done: false, due };
        self.next_id += 1;
        self.todos.push(todo);
        self.save();
        println!("Added todo #{}", self.next_id - 1);
    }

    fn list(&self, show_all: bool) {
        if self.todos.is_empty() {
            println!("No todos. Add one with: todo-rs add \"buy milk\"");
            return;
        }

        let iter: Box<dyn Iterator<Item = &Todo>> = if show_all {
            Box::new(self.todos.iter())
        } else {
            Box::new(self.todos.iter().filter(|t| !t.done))
        };

        for todo in iter {
            let status = if todo.done { "✓" } else { " " };
            let due = todo.due.as_ref().map(|d| format!(" [due: {}]", d)).unwrap_or_default();
            println!("  [{}] #{:<4} {}{}", status, todo.id, todo.title, due);
        }

        let total = self.todos.len();
        let done = self.todos.iter().filter(|t| t.done).count();
        println!("\n  {}/{} done", done, total);
    }

    fn done(&mut self, id: u64) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.done = true;
            self.save();
            println!("Done: #{} - {}", id, todo.title);
        } else {
            println!("Todo #{} not found", id);
        }
    }

    fn undo(&mut self, id: u64) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.done = false;
            self.save();
            println!("Reopened: #{} - {}", id, todo.title);
        } else {
            println!("Todo #{} not found", id);
        }
    }

    fn delete(&mut self, id: u64) {
        if let Some(pos) = self.todos.iter().position(|t| t.id == id) {
            let todo = self.todos.remove(pos);
            self.save();
            println!("Deleted: #{} - {}", id, todo.title);
        } else {
            println!("Todo #{} not found", id);
        }
    }

    fn clear_done(&mut self) {
        let before = self.todos.len();
        self.todos.retain(|t| !t.done);
        let after = self.todos.len();
        self.save();
        println!("Cleared {} completed todos", before - after);
    }
}

fn main() {
    let mut app = TodoApp::load();
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: todo-rs <command> [args]");
        eprintln!("Commands:");
        eprintln!("  add \"title\" [--due YYYY-MM-DD]  Add a new todo");
        eprintln!("  list [--all]                    List todos");
        eprintln!("  done <id>                       Mark todo as done");
        eprintln!("  undo <id>                       Reopen a todo");
        eprintln!("  delete <id>                     Delete a todo");
        eprintln!("  clear                           Clear all done todos");
        return;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("Usage: todo-rs add \"title\"");
                return;
            }
            let title = args[2..].join(" ");
            let due = if let Some(pos) = args.iter().position(|a| a == "--due") {
                args.get(pos + 1).cloned()
            } else {
                None
            };
            app.add(title, due);
        }
        "list" => {
            let show_all = args.contains(&"--all".to_string());
            app.list(show_all);
        }
        "done" => {
            if let Some(id) = args.get(2).and_then(|s| s.parse::<u64>().ok()) {
                app.done(id);
            }
        }
        "undo" => {
            if let Some(id) = args.get(2).and_then(|s| s.parse::<u64>().ok()) {
                app.undo(id);
            }
        }
        "delete" => {
            if let Some(id) = args.get(2).and_then(|s| s.parse::<u64>().ok()) {
                app.delete(id);
            }
        }
        "clear" => {
            app.clear_done();
        }
        _ => {
            eprintln!("Unknown command: {}", args[1]);
        }
    }
}
