mod characters;

use std::env;  // gets the values passed to main()
use characters::*;

fn main() {
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

    let special_characters: &str = "?!@#$%&*()-_=|+[];:><";
    let number_characters: &str = "0123456789";
    let lower_case_letter_characters: &str = "abcdefghijklmnopqrstuvwxyz";
    let capital_letter_characters: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let letters_len: usize = lower_case_letter_characters.len();
    let numbers_len: usize = number_characters.len();
    let spe_cha_len: usize = special_characters.len();

    for _ in 0 .. pass_len {
        let char_type: usize = get_char_type();
        match char_type {
            1 => {
                password.push_str(get_char(special_characters, spe_cha_len));
            }
            2 => {
                password.push_str(get_char(number_characters, numbers_len));
            }
            3 => {
                password.push_str(get_char(lower_case_letter_characters, letters_len));
            }
            _ => {
                password.push_str(get_char(capital_letter_characters, letters_len));
            }
        }
    }

    println!("{}", password);
}
