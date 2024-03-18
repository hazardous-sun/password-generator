mod lib;

use lib;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = match lib::Config::build(&args) {
        Ok(config) => config,
        Err(e) => panic!(e)
    };
}

fn caller() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        eprintln!("ERROR: No parameters passed!\nUsage: passgen [PASSWORD_LEN]");
        std::process::exit(1);
    }

    let pass_len: i32;

    let first_arg = &args[1];
    match first_arg.parse::<i32>() {
        Ok(number) => {
            pass_len = number;
        }
        Err(_) => {
            eprintln!("ERROR: Insert a valid integer!");
            std::process::exit(1);
        }
    }

    let mut previous_characters = PreviousCharacters::new();
    let mut password: String = String::from("");

    let special_characters: &str = "?!@#$%&_|;:";
    let number_characters: &str = "0123456789";
    let lower_case_letter_characters: &str = "abcdefghijklmnopqrstuvwxyz";
    let capital_letter_characters: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let letters_len: usize = lower_case_letter_characters.len();
    let numbers_len: usize = number_characters.len();
    let spe_cha_len: usize = special_characters.len();

    for _ in 0..pass_len {
        let char_type: usize = get_char_type();
        match char_type {
            1 => {
                add_value(
                    &mut password,
                    special_characters,
                    spe_cha_len,
                    &mut previous_characters,
                );
            }
            2 => {
                add_value(
                    &mut password,
                    number_characters,
                    numbers_len,
                    &mut previous_characters,
                );
            }
            3 => {
                add_value(
                    &mut password,
                    lower_case_letter_characters,
                    letters_len,
                    &mut previous_characters,
                );
            }
            _ => {
                add_value(
                    &mut password,
                    capital_letter_characters,
                    letters_len,
                    &mut previous_characters,
                );
            }
        }
    }

    println!("{}", password);
}
