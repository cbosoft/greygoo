use std::fs::File;
use std::io::prelude::*;

pub fn get_contents(filename: &str) -> Result<String, String> {
    if let Ok(mut file) = File::open(filename) {
        let mut contents = String::new();
        if let Ok(_) = file.read_to_string(&mut contents) {
            Ok(contents)
        }
        else {
            Err("failed to read file".to_string())
        }
    }
    else {
        Err("failed to open file".to_string())
    }
}