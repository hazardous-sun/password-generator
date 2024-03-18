mod lib;

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = lib::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("ERROR: {err}");
        process::exit(1);
    });

    if let Err(err) = lib::run(config) {
        eprintln!("ERROR: {err}");
        process::exit(1);
    }
}
