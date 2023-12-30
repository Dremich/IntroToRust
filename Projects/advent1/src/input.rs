use std::io::prelude::*;
use std::fs::File;

/// Function to return a Vector of Strings containing each line
/// of a .txt file with the given name from the current environment
pub(crate) fn get_lines(filename: &str) -> Vec<String>{
    let filepath  =  String::from(concat!(env!("CARGO_MANIFEST_DIR"), "/src/")) + filename;
    let mut file = File::open(filepath).expect("Error opening file");
    let mut buffer = String::new();

    file.read_to_string(&mut buffer).expect("Error reading file");

    buffer.lines().map(String::from).collect()
}