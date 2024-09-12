use std::fs::File;
use std::path::Path;
pub fn read_json_file(filename:String) {
    let path = Path::new(&filename);
    let mut fdata = String::new();
    let mut rfile = File::open(path).expect("File not found");
    rfile.read_to_string(&mut fdata).expect("File can't be read");
}