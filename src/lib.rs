#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "ill";
        let contents = "\
This is a sample string:
The purpose of this text is for testing.
I am still learning.";

        assert_eq!(vec!["I am still learning."], sensitive_search(query, contents));
    }

    #[test]
    fn case_insenstive() {
        let query = "EJO";
        let contents = "\
No tengo mucho que justificar.
La que cambia así es la de Fausto viejo.
Un poco al aire libre.
Hasta ahí lo dejo.";
        assert_eq!(
            vec!["La que cambia así es la de Fausto viejo.", "Hasta ahí lo dejo."],
            insensitive_search(query, contents)
        );
    }
}

use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // Error values: &'static str
            return Err("Not enough arguments!");
        }
        
        let config = Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        };
        
        Ok(config)
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(&config.file_path)?;

    let results = if config.ignore_case == false {
        insensitive_search(&config.query, &content)
    } else {
        sensitive_search(&config.query, &content)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn sensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_uppercase().contains(&query.to_uppercase()) {
            results.push(line)
        }
    }
    
    results
}