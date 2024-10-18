use std::{fs, path::Path};

pub fn open(path: impl AsRef<Path>) -> String {
    let file: String = fs::read_to_string(path).expect("Couldn't open file.");

    file
}
