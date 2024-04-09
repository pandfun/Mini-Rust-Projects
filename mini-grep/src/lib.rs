use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let file_contents = fs::read_to_string(&config.filename)?;

    let result = if config.case_sensitive {
        search_case_sensitive(&config.query, &file_contents)
    } else {
        search_case_insensitive(&config.query, &file_contents)
    };

    println!("");
    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {

        if args.len() != 3 {
            return Err("Not enough arguments!");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        
        // returns true if "CASE" is not an env
        let case_sensitive = !env::var("CASE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut result = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }

    result
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let query = query.to_lowercase();
    let mut result = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "for";
        let contents = "\
this is a sample file content.
this is for the test 1
we are not including this FOR
or this fOr
it only includes case sensitive for";

        assert_eq!(vec!["this is for the test 1", "it only includes case sensitive for"], search_case_sensitive(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "for";
        let contents = "\
this is a sample file content.
this is for the test 1
we are not including this FOR
or this fOr
it only includes case sensitive for";

        assert_eq!(vec!["this is for the test 1", "we are not including this FOR", "or this fOr", "it only includes case sensitive for"], search_case_insensitive(query, contents));
    }
}