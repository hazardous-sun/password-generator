use std::env;  // gets the values passed to main()
use rand::Rng; // generates random values

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

    let mut password: String = String::from("");

    let special_characters: &str = "?!@#$%&*()-_=|+[];:><";
    let number_characters: &str = "0123456789";
    let lower_case_letter_characters: &str = "abcdefghijklmnopqrstuvwxyz";
    let capital_letter_characters: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let letters_len: usize = lower_case_letter_characters.len();
    let numbers_len: usize = number_characters.len();
    let spe_cha_len: usize = special_characters.len();

    for _ in 0 .. pass_len {
        let char_type: usize = rand::thread_rng().gen_range(1..5);
        match char_type {
            1 => {
                let random_spe_cha: usize = rand::thread_rng().gen_range(0..spe_cha_len);
                password.push_str(&special_characters[random_spe_cha..random_spe_cha + 1])
            }
            2 => {
                let random_number: usize = rand::thread_rng().gen_range(0..numbers_len);
                password.push_str(&number_characters[random_number..random_number + 1]);
            }
            3 => {
                let random_letter: usize = rand::thread_rng().gen_range(0..letters_len);
                password.push_str(&lower_case_letter_characters[random_letter..random_letter + 1]);
            }
            _ => {
                let random_letter: usize = rand::thread_rng().gen_range(0..letters_len);
                password.push_str(&capital_letter_characters[random_letter..random_letter + 1]);
            }
        }
    }

    println!("{}", password);
}
