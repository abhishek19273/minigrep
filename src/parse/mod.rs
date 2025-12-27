#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_name: String,
}

impl Config {
    pub fn new(args: &[String]) -> Self {
        let query = args
            .get(1)
            .expect("Please provide the query to search in the file name correctly.");
        let file_name = args
            .get(2)
            .expect("Please provide the file_name correctly to search for the query.");
        Self {
            query: query.clone(),
            file_name: file_name.clone(),
        }
    }
}
pub fn parse_config(args: &[String]) -> Config {
    Config::new(args)
}
