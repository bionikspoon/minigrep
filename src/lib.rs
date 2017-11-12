use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

#[derive(PartialEq, Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Self { query, filename })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }

    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_config() {
        let args = [
            String::from(""),
            String::from("hello"),
            String::from("text.txt"),
        ];
        let actual = Config::new(&args);
        let expected = Ok(Config {
            query: String::from("hello"),
            filename: String::from("text.txt"),
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn it_handles_bad_args() {
        let args = [String::from("")];
        let actual = Config::new(&args);
        let expected = Err("not enough arguments");

        assert_eq!(actual, expected);
    }

    #[test]
    fn search_has_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
