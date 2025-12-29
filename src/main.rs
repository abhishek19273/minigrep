use minigrep::search;
use std::{env, error::Error, fs, process};
mod parse;
use parse::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = match Config::build(&args) {
        Ok(val) => val,
        Err(s) => {
            println!("{s}");
            process::exit(1);
        }
    };
    match run(config) {
        Ok(_) => process::exit(0),
        Err(_) => process::exit(1),
    }
}
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("The pattern to search for is {}", config.query);
    println!("The file to search this pattern is {}", config.file_name);
    let file_content = fs::read_to_string(&config.file_name)?;
    println!("The file content is {file_content}");
    for line in search(&config.query, &file_content) {
        println!("{line}");
    }
    Ok(())
}
