use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader, BufWriter};
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
        TodoList { items: Vec::new() }
    }

    fn add(&mut self, description: String) {
        let id = self.items.len() + 1;
        self.items.push(TodoItem {
            id,
            description,
            completed: false,
        });
    }

    fn delete(&mut self, id: usize) {
        if let Some(index) = self.items.iter().position(|i| i.id == id) {
            self.items.remove(index);
        } else {
            println!("Task with id {} not found.", id);
        }
    }

    fn edit(&mut self, id: usize, description: String) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.description = description;
        } else {
            println!("Task with id {} not found.", id);
        }
    }

    fn complete(&mut self, id: usize) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.completed = true;
        } else {
            println!("Task with id {} not found.", id);
        }
    }

    fn uncomplete(&mut self, id: usize) {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.completed = false;
        } else {
            println!("Task with id {} not found.", id);
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

    fn clear(&mut self) {
        self.items.clear();
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

fn main() {
    let filename = "todos.json";
    let mut todo_list = TodoList::load(filename).expect("Failed to load todo list");

    loop {
        println!("What do you want to do?");
        println!("1. Add task");
        println!("2. Complete task");
        println!("3. List tasks");
        println!("4. Save and exit");

        let mut choice: String = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = choice.trim().parse().expect("Please enter a number");
        match choice {
            1 => {
                println!("Enter task description:");
                let mut description = String::new();
                io::stdin()
                    .read_line(&mut description)
                    .expect("Failed to read line");
                todo_list.add(description.trim().to_string());
            }
            2 => {
                println!("Enter task id to complete");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: usize = id.trim().parse().expect("Please enter a number");
                todo_list.complete(id);
            }
            3 => todo_list.list(),
            4 => {
                todo_list.save(filename).expect("Failed to save todo list");
                break;
            }
            _ => println!("Invalid choice, please try again"),
        }
    }
}
