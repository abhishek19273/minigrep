pub fn search<'a>(query: &str, file_content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in file_content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, file_content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in file_content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod test {
    use crate::{search, search_case_insensitive};

    #[test]
    fn one_result() {
        let query = "duct";
        let file_content = "\
Rust:
safe, fast, productive
Pick three.";
        assert_eq!(vec!["safe, fast, productive"], search(query, file_content));
    }

    #[test]
    fn case_insensitive() {
        let query = "DuCt";
        let file_content = "\
Rust:
safe, fast, prodUcTive
Pick three.";
        assert_eq!(
            vec!["safe, fast, prodUcTive"],
            search_case_insensitive(query, file_content)
        );
    }
}
