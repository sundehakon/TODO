use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub text: String
}

pub fn read_json_file(filename:String) {
    let path = Path::new(&filename);
    let mut fdata = String::new();
    let mut rfile = File::open(path).expect("File not found");
    rfile.read_to_string(&mut fdata).expect("File can't be read");
    let todos: Vec<Todo> = serde_json::from_str(&fdata).unwrap();
    for todo in todos {
        println!("TODO: {}", todo.text)
    }
}

pub fn write_json_file(filename: String, todos: &Vec<Todo>) {
    let mut file = File::create(filename).expect("Unable to create file");
    let json = serde_json::to_string_pretty(&todos).expect("Unable to serialize data");
    file.write_all(json.as_bytes()).expect("Unable to write data to file");
}