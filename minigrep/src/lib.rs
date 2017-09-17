use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename).expect("file not found.");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_insensitive {
        search(&config.query, &contents)
    } else {
        search_case_sensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
    case_insensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_insensitive = env::var("CASE_SENSITIVE").is_err();

        Ok(Config { filename, query, case_insensitive })
    }
}

pub fn search_case_sensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}


pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(&query) {
            results.push(line);
        }
    }
    results
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn open_result() {
        let query = "duct";
        let contents = "
Rust:
safe, fast, productive
Pick three.
Duct tape";
        assert_eq!(vec!("safe, fast, productive"), search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rust";
        let contents = "
Rust:
safe, fast, productive
Pick three.
Trust me";
        assert_eq!(vec!["Rust:", "Trust me"], search_case_sensitive(query, contents));

    }

}
