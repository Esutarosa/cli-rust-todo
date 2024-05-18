use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct TodoItem {
    id: String,
    description: String,
    completed: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoList {
    items: Vec<TodoItem>,
}

fn main() {}
