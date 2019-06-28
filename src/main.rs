use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Could not parse args: {}", err);

        println!("Could not parse args: {}", err);
        process::exit(1)
    });

    if let Err(err) = minigrep::run(config) {
        println!("could not start: {}", err);
        process::exit(1)
    }
}
