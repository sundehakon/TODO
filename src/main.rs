mod modules;
use modules::readjson::{Todo, Quote};
use std::io;
use rand::seq::SliceRandom;

fn main() {
        println!("Welcome to your personal To-Do list!");
    loop {
        println!("Choose an option:");
        println!("1. Add a new todo");
        println!("2. Remove a todo");
        println!("3. Display todo's");
        println!("4. Need motivation?");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                let new_todo = modules::readjson::get_user_input();

                let filename = "src/data.json".to_string();
                let mut todos: Vec<Todo> = modules::readjson::read_json_file(filename.clone());
                todos.push(new_todo);
                modules::readjson::write_json_file(filename, &todos);
            }
            "2" => {
                let filename = "src/data.json".to_string();
                let todos: Vec<Todo> = modules::readjson::read_json_file(filename.clone());

                if todos.is_empty() {
                    println!("No todo's to remove.");
                    continue;
                }

                println!("Here are your todo's:");
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

                modules::readjson::remove_todo_by_index(filename, index);
            }
            "3" => {
                let filename = "src/data.json".to_string();
                let todos: Vec<Todo> = modules::readjson::read_json_file(filename);

                if todos.is_empty() {
                    println!("No todos available.");
                } else {
                    println!("Here are your todo's:");
                    for (index, todo) in todos.iter().enumerate() {
                        println!("{}. {}", index + 1, todo.text);
                    }
                }
            }
            "4" => {
                let filename = "src/quotes.json".to_string();
                let quotes: Vec<Quote> = modules::readjson::read_json_file(filename.clone());

                if quotes.is_empty() {
                    println!("No quotes available.");
                } else {
                    let random_quote = quotes.choose(&mut rand::thread_rng());
                    match random_quote {
                        Some(quote) => println!("\"{}\" - {}", quote.quote, quote.author),
                        None => println!("Could not retrieve a random quote."),
                    }
                }
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}
