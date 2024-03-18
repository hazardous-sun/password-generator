use std::{env, process};

use passgen::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    if let Err(err) = passgen::run(config) {
        eprintln!("{err}");
        process::exit(1);
    }
}
