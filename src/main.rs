use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    id: u64,
    name: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
    id: u64,
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Database {
    tasks: HashMap<u64, Task>,
    users: HashMap<u64, User>,
}

impl Database {
    fn new() -> Self {
        Self {
            tasks: HashMap::new(),
            users: HashMap::new(),
        }
    }

    fn insert_task(&mut self, task: Task) {
        self.tasks.insert(task.id, task);
    }

    fn insert_user(&mut self, user: User) {
        self.users.insert(user.id, user);
    }
}

fn main() {
    println!("Hello, world!!!");
}
