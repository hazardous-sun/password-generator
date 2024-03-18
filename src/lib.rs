use rand::Rng;
use std::env;
use std::error::Error;

pub struct Config {
    len: i32,
    upper_case: bool,
    lower_case: bool,
    numbers: bool,
    math_symbols: bool,
    extra_symbols: bool,
    check_repetition: bool
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("No parameters passed!\nUsage: passgen [PASSWORD_LEN]");
        }

        let pass_len: i32;

        let first_arg = &args[1];
        match first_arg.parse::<i32>() {
            Ok(number) => {
                pass_len = number;
            }
            Err(_) => {
                return Err("Insert a valid integer for the password length.");
            }
        }

        return Ok(Config{
            len: pass_len,
            upper_case: env::var("UPPER_CASE").is_ok(),
            lower_case: env::var("LOWER_CASE").is_ok(),
            numbers: env::var("NUMBERS").is_ok(),
            math_symbols: env::var("MATH_SYM").is_ok(),
            extra_symbols: env::var("EXTRA_SYM").is_ok(),
            check_repetition: env::var("CHECK_REP").is_ok()
        })
    }
}

struct Symbols {
    upper_case: &'static str,
    lower_case: &'static str,
    numbers: &'static str,
    math_symbols: &'static str,
    extra_symbols: &'static str,
    valid_char: &'static [i8]
}

impl Symbols {
    fn new(config: Config) -> Symbols {
        let upper_case = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let lower_case = "abcdefghijklmnopqrstuvwxyz";
        let numbers = "0123456789";
        let math_symbols = "-+=*/><[]{}()";
        let extra_symbols = "?!@#$%&_|;:";

        let mut valid_char: Vec<i8> = Vec::new();

        if config.upper_case { valid_char.push(1); }
        if config.lower_case { valid_char.push(2); }
        if config.numbers { valid_char.push(3); }
        if config.math_symbols { valid_char.push(4); }
        if config.extra_symbols { valid_char.push(5); }

        Symbols {
            upper_case,
            lower_case,
            numbers,
            math_symbols,
            extra_symbols,
            valid_char: valid_char.as_slice()
        }
    }

    fn get_char(&self, str: &str) -> &char {
        let pos = rand::thread_rng().gen_range(0..str.len() - 1);
        &str.chars().nth(pos).unwrap()
    }

    fn get_char_type(&self) -> usize {
        rand::thread_rng().gen_range(1..5)
    }

    fn add_value(&self) {}
}

struct PreviousCharacters {
    characters: (i8, i8, i8),
}

impl PreviousCharacters {
    pub fn new() -> Self {
        PreviousCharacters {
            characters: (-1, -1, -1),
        }
    }

    pub fn get_characters(&self) -> (i8, i8, i8) {
        self.characters
    }

    pub fn adjust(&mut self, last: i8) {
        let (first, second, _) = self.characters;
        self.characters = (last, first, second);
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut password: String = String::from("");
    let mut previous_characters = PreviousCharacters::new();
    let symbols = Symbols::new(config);

    for _ in 0..config.len {
        let char_type: usize = symbols.get_char_type();
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

pub fn check_repetition(new: i8, last: (i8, i8, i8)) -> i8 {
    let mut repetitions: i8 = 0;
    if new == last.0 {
        repetitions += 1;
    }
    if new == last.1 {
        repetitions += 1;
    }
    if new == last.2 {
        repetitions += 1;
    }
    repetitions
}

pub fn reroll(new: i8, last: (i8, i8, i8)) -> bool {
    let repetitions = check_repetition(new, last);
    if repetitions >= 2 {
        return true;
    }
    false
}

pub fn add_value(
    password: &mut String,
    string: &str,
    len: usize,
    previous: &mut PreviousCharacters,
) {
    let mut new_char = get_char(string, len);
    if reroll(1, previous.get_characters()) {
        new_char = get_char(string, len);
    }
    password.push_str(new_char);
    previous.adjust(1);
}
