use std::{env, process};

use password_generator::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("ERROR: {err}");
        process::exit(1);
    });

    if let Err(err) = password_generator::run(config) {
        eprintln!("ERROR: {err}");
        process::exit(1);
    }
}
