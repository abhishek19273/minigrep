use std::{env, process};
mod parse;
mod read_file;
use parse::Config;
use read_file::read_file_content;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = match Config::build(&args) {
        Ok(val) => val,
        Err(s) => {
            println!("{s}");
            process::exit(1);
        }
    };
    println!("The pattern to search for is {}", config.query);
    println!("The file to search this pattern is {}", config.file_name);
    let file_content = read_file_content(&config.file_name);
    println!("The file content is {file_content}");
}
