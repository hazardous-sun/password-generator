use std::{env, process};

use passgen::Config;

/// Entry point for the password generator application.
///
/// This function performs the following steps:
/// 1. Gathers command-line arguments as a vector of strings using `env::args().collect()`.
/// 2. Attempts to build a `Config` instance from the arguments using `Config::build`.
///     - If `Config::build` encounters an error, it prints the error message using `eprintln!` and exits the process with an exit code of 1.
/// 3. Attempts to run the password generation process using `passgen::run(config)`.
///     - If `passgen::run` encounters an error, it prints the error message using `eprintln!` and exits the process with an exit code of 1.
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
