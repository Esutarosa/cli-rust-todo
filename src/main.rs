use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct TodoItem {
    id: usize,
    description: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoList {
    items: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn add(&mut self, description: String) {
        let id: usize = self.items.len() + 1;
        self.items.push(TodoItem {
            id,
            description,
            completed: false,
        });
    }

    fn complete(&mut self, id: usize) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.completed = true;
        }
    }

    fn list(&self) {
        for item in &self.items {
            println!(
                "{}: {} [{}]",
                item.id,
                item.description,
                if item.completed { "x" } else { " " }
            );
        }
    }

    fn save(&self, filename: &str) -> io::Result<()> {
        let file = File::create(filename)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self)?;
        Ok(())
    }

    fn load(filename: &str) -> io::Result<Self> {
        if Path::new(filename).exists() {
            let file = File::open(filename)?;
            let reader = BufReader::new(file);
            let list = serde_json::from_reader(reader)?;
            Ok(list)
        } else {
            Ok(TodoList::new())
        }
    }
}

fn main() {}
