mod modules;
use modules::readjson;

fn main() {
    readjson::read_json_file("/src/data.json".to_string());
}
