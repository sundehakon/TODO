mod modules;
use modules::readjson;
use std::{io, usize};

fn main() {
    loop {
        println!("Welcome to your personal To-Do list!");
        println!("Choose an option:");
        println!("1. Add a new todo");
        println!("2. Remove a todo");
        println!("3. Display todos");
        println!("4. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                let new_todo = readjson::get_user_input();

                let filename = "src/data.json".to_string();
                let mut todos = readjson::read_json_file(filename.clone());
                todos.push(new_todo);
                readjson::write_json_file(filename, &todos);
            }
            "2" => {
                let filename = "src/data.json".to_string();
                let todos = readjson::read_json_file(filename.clone());

                if todos.is_empty() {
                    println!("No todos to remove.");
                    continue;
                }

                println!("Here are your todos:");
                for (index, todo) in todos.iter().enumerate() {
                    println!("{}. {}", index + 1, todo.text);
                }

                println!("Enter the number of the todo you want to remove:");

                let mut index = String::new();
                io::stdin().read_line(&mut index).expect("Failed to read line");
                let index: usize = match index.trim().parse::<usize>() {
                    Ok(num) => num - 1,
                    Err(_) => {
                        println!("Invalid input.");
                        continue;
                    }
                };

                readjson::remove_todo_by_index(filename, index);
            }
            "3" => {
                let filename = "src/data.json".to_string();
                let todos = readjson::read_json_file(filename);

                if todos.is_empty() {
                    println!("No todos available.");
                } else {
                    println!("Here are your todos:");
                    for (index, todo) in todos.iter().enumerate() {
                        println!("{}. {}", index + 1, todo.text);
                    }
                }
            }
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}
