use rand::Rng;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct Config {
    len: i32,
    upper: bool,
    lower: bool,
    numbers: bool,
    basic_sym: bool,
    extra_sym: bool,
    check_rep: bool,
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

        let mut upper = false;
        let mut lower = false;
        let mut numbers = false;
        let mut basic_sym = false;
        let mut extra_sym = false;
        let mut check_rep = false;

        'flags: for flag in args.iter() {
            match flag.to_lowercase().as_str() {
                "--all" | "-a" => {
                    upper = true;
                    lower = true;
                    numbers = true;
                    basic_sym = true;
                    extra_sym = true;
                    check_rep = true;
                    break 'flags;
                },
                "--upper" | "-u" => { upper = true; },
                "--lower" | "-l" => { lower = true; },
                "--numbers" | "-n" => { numbers = true; },
                "--basic-sym" | "-b" => { basic_sym = true; },
                "--extra-sym" | "-e" => { extra_sym = true; },
                "--check-rep" | "-r" => { check_rep = true; },
                "--help" | "-h" => {
                    return Err("Usage: passgen [PASS_GEN] [OPTIONS]\n-a -u -l -n -b -e -r");
                },
                _ => ()
            }
        }

        return Ok(Config {
            len: pass_len,
            upper,
            lower,
            numbers,
            basic_sym,
            extra_sym,
            check_rep,
        });
    }
}

struct Symbols {
    characters: Vec<&'static str>,
}

impl Symbols {
    fn new(config: &Config) -> Symbols {
        let mut characters: Vec<&'static str> = Vec::new();

        if config.upper { characters.push("ABCDEFGHIJKLMNOPQRSTUVWXYZ"); }
        if config.lower { characters.push("abcdefghijklmnopqrstuvwxyz"); }
        if config.numbers { characters.push("0123456789"); }
        if config.basic_sym { characters.push("-+=*/><[]{}()"); }
        if config.extra_sym { characters.push("?!@#$%&_|;:"); }

        if characters.len() < 1 {
            characters.push("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
            characters.push("abcdefghijklmnopqrstuvwxyz");
            characters.push("0123456789");
            characters.push("-+=*/><[]{}()");
            characters.push("?!@#$%&_|;:");
        }

        Symbols {
            characters,
        }
    }

    fn get_char(&self, str: &'static str) -> char {
        let pos = rand::thread_rng().gen_range(0..str.len() - 1);
        str.chars().nth(pos).unwrap()
    }

    fn get_char_type(&self) -> (&'static str, i8) {
        let mut char_type;

        if self.characters.len() > 1 {
            char_type = rand::thread_rng().gen_range(0..=self.characters.len() - 1);
            return (self.characters[char_type], char_type as i8);
        }

        (self.characters[0], 0)
    }

    fn add_value(
        &self,
        config: &Config,
        password: &mut String,
        str: &'static str,
        char_type: i8,
        previous: &mut PreviousCharacters,
    ) {
        let mut new_char: char = self.get_char(str);
        if config.check_rep && previous.reroll(char_type) {
            new_char = self.get_char(str);
        }
        password.push(new_char);
        previous.adjust(1);
    }
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

    fn adjust(&mut self, last: i8) {
        let (first, second, _) = self.characters;
        self.characters = (last, first, second);
    }

    fn check_repetition(&self, last: (i8, i8, i8), new: i8) -> i8 {
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

    fn reroll(&self, new: i8) -> bool {
        let repetitions = self.check_repetition(self.characters, new);
        if repetitions >= 2 {
            return true;
        }
        false
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut password: String = String::from("");
    let mut previous_characters = PreviousCharacters::new();
    let symbols = Symbols::new(&config);

    for _ in 0..config.len {
        let (str, char_type) = symbols.get_char_type();
        symbols.add_value(
            &config,
            &mut password,
            str,
            char_type,
            &mut previous_characters,
        )
    }

    println!("{}", password);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_build() {
        let program_name = String::from("program name");
        let password_name = String::from("16");
        let o1 = String::from("-u");
        let o2 = String::from("--lower");
        let o3 = String::from("-r");

        let args = vec![
            program_name,
            password_name,
            o1,
            o2,
            o3
        ];

        let config = Config {
            len: 16,
            upper: true,
            lower: true,
            numbers: false,
            basic_sym: false,
            extra_sym: false,
            check_rep: true
        };

        assert_eq!(config, Config::build(&args).expect("Config not correctly built"));
    }
}
