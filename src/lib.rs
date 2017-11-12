use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::iter::Iterator;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Env {
    pub case_sensitive: bool,
}

impl Env {
    pub fn new() -> Self {
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Self { case_sensitive }
    }
}

#[derive(PartialEq, Debug)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new<T>(env: &Env, mut args: T) -> Result<Self, &'static str>
    where
        T: Iterator<Item = String>,
    {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get a file name"),
        };


        Ok(Self {
            query,
            filename,
            case_sensitive: env.case_sensitive,
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()

}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn it_creates_config() {
    //     let mut args = [
    //         String::from(""),
    //         String::from("hello"),
    //         String::from("text.txt"),
    //     ].into_iter();
    //     let env = Env { case_sensitive: true };
    //     let actual = Config::new(&env, args);
    //     let expected = Ok(Config {
    //         query: String::from("hello"),
    //         filename: String::from("text.txt"),
    //         case_sensitive: true,
    //     });
    //
    //     assert_eq!(actual, expected);
    // }


    // #[test]
    // fn it_handles_bad_args() {
    //     let mut args = [String::from("")].into_iter();
    //     let env = Env { case_sensitive: true };
    //     let actual = Config::new(&env, args);
    //     let expected = Err("not enough arguments");
    //
    //     assert_eq!(actual, expected);
    // }

    #[test]
    fn search_has_one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }



    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }


}
