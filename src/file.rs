use std::fs;

pub fn open_file(path: &str) -> String {
    let file: String = fs::read_to_string(path).expect("Couldn't open file.");

    file
}
