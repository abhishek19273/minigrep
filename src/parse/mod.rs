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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_build_positive() {
        let input: Vec<String> = vec![
            String::from("minigrep"),
            String::from("hello"),
            String::from("poem.txt"),
        ];
        let config: Result<Config, &'static str> = Config::build(&input);
        match config {
            Ok(val) => {
                assert_eq!(val.query, "hello");
                assert_eq!(val.file_name, "poem.txt");
            }
            Err(e) => {
                panic!("Reason for failure {e}");
            }
        }
    }

    #[test]
    fn config_build_negative() {
        let input: Vec<String> = vec![String::from("minigrep"), String::from("hello")];
        let config: Result<Config, &'static str> = Config::build(&input);
        match config {
            Ok(val) => {
                panic!("The config value is {val:?}");
            }
            Err(e) => {
                assert_eq!(e.to_string(), "Not Enough Arguments.");
            }
        }
    }
}
