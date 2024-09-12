mod modules;
use modules::readjson;
use std::io;

fn main() {
    println!("Welcome to your personal To-Do list!");
    println!("1. See To-Do list");
    println!("2. Write new To-Do");
    // Add logic for removing items from list
    println!("3. Remove from To-Do list");

    let mut input = String::new();
    println!("Choose your operation:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number");
            return;
        }
    };

    if input > 3 {
        println!("Please choose a number from the list!");
    }

    if input == 1 {
        let todos = readjson::read_json_file("src/data.json".to_string());
        for todo in todos {
            println!("Todo: {}", todo.text);
        }
    }

    if input == 2 {
        let new_todo = readjson::get_user_input();
        readjson::write_json_file("src/data.json".to_string(), new_todo);
    }
}
