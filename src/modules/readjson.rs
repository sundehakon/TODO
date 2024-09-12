use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;
#[derive(Serialize, Deserialize)]

struct Todo {
    text: String
}

pub fn read_json_file(filename:String) {
    let path = Path::new(&filename);
    let mut fdata = String::new();
    let mut rfile = File::open(path).expect("File not found");
    rfile.read_to_string(&mut fdata).expect("File can't be read");
    let todos: Vec<Todo> = serde_json::from_str(&fdata).unwrap();
    for todo in todos {
        println!("Text: {}", todo.text)
    }
}