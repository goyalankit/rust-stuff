use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename).expect("file not found.");
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { filename, query })
    }
}

pub fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
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
Pick three. ";
        assert_eq!(vec!("safe, fast, productive"), search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let contents = "
Rust: 
safe, fast, productive
Pick three.
Trust me";
        assert_eq!(vec!["Rust:", "Trust me"], search(query, contents));
    
    }

}
