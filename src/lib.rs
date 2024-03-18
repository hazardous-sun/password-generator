use rand::Rng;
use std::env;
use std::error::Error;

// struct that will be used to run the program
pub struct Config {
    len: i32,
    upper_case: bool,
    lower_case: bool,
    numbers: bool,
    math_symbols: bool,
    extra_symbols: bool
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("ERROR: No parameters passed!\nUsage: passgen [PASSWORD_LEN]");
        }

        let pass_len: i32;

        let first_arg = &args[1];
        match first_arg.parse::<i32>() {
            Ok(number) => {
                pass_len = number;
            }
            Err(_) => {
                return Err("ERROR: Insert a valid integer for the password length.");
            }
        }

        return Ok(Config{
            len: pass_len,
            upper_case: env::var("UPPER_CASE").is_ok(),
            lower_case: env::var("LOWER_CASE").is_ok(),
            numbers: env::var("NUMBERS").is_ok(),
            math_symbols: env::var("MATH_SYM").is_ok(),
            extra_symbols: env::var("EXTRA_SYM").is_ok()
        })
    }
}

struct Symbols {
    upper_case: &'static str,
    lower_case: &'static str,
    numbers: &'static str,
    math_symbols: &'static str,
    extra_symbols: &'static str,
}

impl Symbols {
    fn build() -> Symbols {
        Symbols {
            upper_case: "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            lower_case: "abcdefghijklmnopqrstuvwxyz",
            numbers: "0123456789",
            math_symbols: "-+=*/><[]{}()",
            extra_symbols: "?!@#$%&_|;:"
        }
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

}

pub struct PreviousCharacters {
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

pub fn get_char_type() -> usize {
    rand::thread_rng().gen_range(1..5)
}

pub fn get_char(str: &str, str_len: usize) -> &str {
    let pos = rand::thread_rng().gen_range(0..str_len);
    &str[pos..pos + 1]
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
