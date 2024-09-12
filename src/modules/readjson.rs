use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::io;

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub text: String,
}

pub fn read_json_file(filename: String) -> Vec<Todo> {
    let path = Path::new(&filename);
    
    if !path.exists() {
        println!("File not found, starting with an empty list.");
        return vec![];
    }

    let mut fdata = String::new();
    let mut rfile = File::open(path).expect("Unable to open file");
    rfile.read_to_string(&mut fdata).expect("Unable to read file");
    
    serde_json::from_str(&fdata).unwrap_or_else(|_| vec![])
}

pub fn write_json_file(filename: String, todos: &Vec<Todo>) {
    let mut file = File::create(filename).expect("Unable to create file");
    let json = serde_json::to_string_pretty(&todos).expect("Unable to serialize data");
    file.write_all(json.as_bytes()).expect("Unable to write data to file");

    println!("Todos updated successfully!");
}

pub fn get_user_input() -> Todo {
    println!("Enter text:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    Todo {
        text: input.trim().to_string(),
    }
}

pub fn remove_todo_by_index(filename: String, index: usize) {
    let mut todos = read_json_file(filename.clone());

    if index >= todos.len() {
        println!("Invalid index.");
        return;
    }

    todos.remove(index);

    write_json_file(filename, &todos);
}
