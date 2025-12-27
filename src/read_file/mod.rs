use std::fs;

pub fn read_file_content(file_name: &String) -> String {
    fs::read_to_string(file_name).expect("Unable to read the file.")
}
