mod modules;
use modules::readjson::{self, Todo};

fn main() {
    let todos = vec![
        Todo { text: String::from("Finish projects")}
    ];

    readjson::read_json_file("src/data.json".to_string());
    readjson::write_json_file("src/data.json".to_string(), &todos);
}
