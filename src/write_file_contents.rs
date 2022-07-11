use std::fs::write;

pub fn write_contents(filename: &str, contents: &str) -> Result<(), String> {
    match write(filename, contents) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("failed to write contents of file \"{filename}\""))
    }
}