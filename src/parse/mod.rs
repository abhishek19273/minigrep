#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_name: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments.");
        }
        let query = args
            .get(1)
            .expect("Please provide the query to search in the file name correctly.");
        let file_name = args
            .get(2)
            .expect("Please provide the file_name correctly to search for the query.");
        Ok(Self {
            query: query.clone(),
            file_name: file_name.clone(),
        })
    }
}
