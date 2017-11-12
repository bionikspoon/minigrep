extern crate minigrep;

use std::env;
use std::process;

use minigrep::{Config, Env};

fn main() {
    let env = Env::new();

    let config = Config::new(&env, env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
