use rand::Rng;
use std::error::Error;

/// Configuration for password generation
#[derive(Debug, PartialEq)]
pub struct Config {
    /// Desired length of the password
    len: i32,
    /// Flag for using upper case letters in the password
    upper: bool,
    /// Flag for using lower case letters in the password
    lower: bool,
    /// Flag for using numbers in the password
    numbers: bool,
    /// Flag for using basic symbols in the password
    basic_sym: bool,
    /// Flag for using extra symbols in the password
    extra_sym: bool,
    /// Flag for checking if there are characters that repeat too much in the password
    check_rep: bool,
}

impl Config {
    /// Builds a `Config` instance from command-line arguments.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use passgen::Config;
    /// let config = Config::build(&["".to_string(), "12".to_string(), "-u".to_string(), "--numbers".to_string()]).unwrap();
    /// ```
    ///
    /// # Errors
    ///
    /// - Returns an error if no arguments are provided.
    /// - Returns an error if the second argument is not a valid integer for password length.
    /// - Returns an error if the `--help` flag is used.
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
                }
                "--upper" | "-u" => { upper = true; }
                "--lower" | "-l" => { lower = true; }
                "--numbers" | "-n" => { numbers = true; }
                "--basic-sym" | "-b" => { basic_sym = true; }
                "--extra-sym" | "-e" => { extra_sym = true; }
                "--check-rep" | "-r" => { check_rep = true; }
                "--help" | "-h" => {
                    return Err("Usage: passgen [PASS_GEN] [OPTIONS]\n-a -u -l -n -b -e -r");
                }
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

/// This struct represents the available character sets for password generation.
struct Symbols {
    /// A vector containing references to character sets as string slices.
    /// Each element can be uppercase letters, lowercase letters, numbers, basic symbols, or extra symbols.
    characters: Vec<&'static str>,
}

impl Symbols {
    /// Creates a `Symbols` instance based on the provided configuration.
    ///
    /// This function iterates over the configuration flags and adds the corresponding character set
    /// to the `characters` vector. If no character sets are selected in the configuration,
    /// it defaults to including all available sets.
    ///
    /// # Arguments
    ///
    /// * `config` - A reference to the `Config` struct containing user-defined password generation options.
    ///
    /// # Returns
    ///
    /// A `Symbols` instance with the character sets based on the configuration.
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

    /// Gets a random character from the provided character set string slice.
    ///
    /// This function uses the `rand::thread_rng` to generate a random index within the bounds of the
    /// character set string slice and returns the character at that position.
    ///
    /// # Arguments
    ///
    /// * `str` - A reference to the character set string slice to get a random character from.
    ///
    /// # Returns
    ///
    /// A random character from the provided character set string slice.
    fn get_char(&self, str: &'static str) -> char {
        let pos = rand::thread_rng().gen_range(0..str.len() - 1);
        str.chars().nth(pos).unwrap()
    }

    /// Gets a random character and its corresponding character type index.
    ///
    /// This function first checks if there are multiple character sets available. If so, it
    /// generates a random index within the range of the `characters` vector length. It then returns
    /// a tuple containing the character set string slice at the chosen index and the index itself
    /// cast as an `i8`. If there's only one character set available, it returns a tuple containing
    /// the single character set and 0 as the character type index.
    ///
    /// # Returns
    ///
    /// A tuple containing a reference to a character set string slice (`&'static str`) and its corresponding character type index (`i8`).
    fn get_char_type(&self) -> (&'static str, i8) {
        let char_type;

        if self.characters.len() > 1 {
            char_type = rand::thread_rng().gen_range(0..=self.characters.len() - 1);
            return (self.characters[char_type], char_type as i8);
        }

        (self.characters[0], 0)
    }

    /// Adds a character to the password based on the provided configuration and character type.
    ///
    /// It first retrieves a random character from the specified character set using `get_char`.
    /// If the `check_rep` flag is enabled in the configuration and `previous.reroll(char_type)` returns true
    /// (indicating a repeated character type), it generates a new random character to prevent repetition.
    /// Finally, it appends the chosen character to the `password` string and updates the `previous` characters state.
    ///
    /// # Arguments
    ///
    /// * `config` - A reference to the `Config` struct containing user-defined password generation options.
    /// * `password` - A mutable string reference to which the new character will be appended.
    /// * `str` - A reference to the character set string slice to get a random character from.
    /// * `char_type` - The index of the character set used (obtained from `get_char_type`).
    /// * `previous` - A reference to a `PreviousCharacters` struct, used for tracking previously used characters.
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

/// This struct keeps track of previously used character types to prevent repetition in password generation.
struct PreviousCharacters {
    /// A tuple containing the last three used character type indices.
    /// The first element represents the most recent character type, the second the second-most recent, and so on.
    characters: (i8, i8, i8),
}

impl PreviousCharacters {
    /// Creates a new `PreviousCharacters` instance with all character types initialized to -1 (not used yet).
    fn new() -> Self {
        PreviousCharacters {
            characters: (-1, -1, -1),
        }
    }

    /// Updates the internal state with the most recently used character type.
    ///
    /// This function shifts the previously used character types one position down the tuple.
    /// The most recent character type is placed at the first element of the tuple.
    ///
    /// # Arguments
    ///
    /// * `last` - The index of the character type that was just used.
    fn adjust(&mut self, last: i8) {
        let (first, second, _) = self.characters;
        self.characters = (last, first, second);
    }

    /// Checks for repetition based on the last three used character types and a new character type.
    ///
    /// This function iterates through the stored character types and compares them to the provided `new`
    /// character type. It increments a counter (`repetitions`) for each match.
    ///
    /// # Arguments
    ///
    /// * `last` - A tuple representing the last three used character type indices.
    /// * `new` - The index of the character type to be checked for repetition.
    ///
    /// # Returns
    ///
    /// An `i8` value representing the number of times the `new` character type appears within the last three used types.
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

    /// Determines if a re-roll (generating a new character) is necessary to prevent repetition.
    ///
    /// This function calls `check_repetition` to determine how many times the new character type would appear
    /// consecutively. It returns `true` if there are two or more repetitions, indicating a need to re-roll
    /// and generate a different character type.
    ///
    /// # Arguments
    ///
    /// * `new` - The index of the character type to be checked for repetition.
    ///
    /// # Returns
    ///
    /// A `bool` value indicating whether a re-roll is necessary (`true`) or not (`false`).
    fn reroll(&self, new: i8) -> bool {
        let repetitions = self.check_repetition(self.characters, new);
        if repetitions >= 2 {
            return true;
        }
        false
    }
}

/// Generates a password based on the provided configuration.
///
/// This function takes a `Config` struct containing user-defined password generation options
/// and attempts to create a password that meets those criteria. It performs the following steps:
///
/// 1. Initializes an empty string to store the password.
/// 2. Creates a new `PreviousCharacters` instance to track previously used character types.
/// 3. Creates a `Symbols` instance based on the provided configuration to manage available character sets.
/// 4. Iterates the desired password length defined in `config.len`.
///     - In each iteration, it calls `Symbols::get_char_type` to get a random character set and its corresponding index.
///     - Then, it calls `Symbols::add_value` to add a random character from the chosen set to the password string.
///     - This function also considers the `check_rep` flag in the configuration and the `PreviousCharacters` state
///       to potentially re-roll the character if repetition needs to be prevented.
/// 5. Finally, it prints the generated password to the console and returns `Ok(())` on success.
///
/// # Arguments
///
/// * `config` - A `Config` struct containing user-defined password generation options.
///
/// # Returns
///
/// A `Result` type. On success, it returns `Ok(())`. On error, it returns an error wrapped in a `Box<dyn Error>`.
pub fn run(config: Config) -> Result<String, Box<dyn Error>> {
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
    Ok(password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_build_valid_args() {
        // Test without flags provided
        let program_name = String::from("program name");
        let password_len = String::from("16");

        let args = vec![program_name.clone(), password_len.clone()];

        let config = Config {
            len: 16,
            upper: false,
            lower: false,
            numbers: false,
            basic_sym: false,
            extra_sym: false,
            check_rep: false,
        };

        assert_eq!(Ok(config), Config::build(&args));

        // Test with flags provided
        let f1 = String::from("-u");
        let f2 = String::from("--lower");
        let f3 = String::from("-r");

        let args = vec![
            program_name,
            password_len,
            f1,
            f2,
            f3,
        ];

        let config = Config {
            len: 16,
            upper: true,
            lower: true,
            numbers: false,
            basic_sym: false,
            extra_sym: false,
            check_rep: true,
        };

        assert_eq!(Ok(config), Config::build(&args));
    }

    #[test]
    fn config_build_invalid_args() {
        // Test with a non-numeric password length:
        let args = vec!["program_name".to_string(), "abc".to_string(), "-u".to_string()];
        assert_eq!("ERROR: Insert a valid integer for the password length.",
                   Config::build(&args).unwrap_err());

        // Test with too few arguments:
        let args = vec!["program_name".to_string()];
        assert_eq!("ERROR: No parameters passed!\nUsage: passgen [PASSWORD_LEN]",
                   Config::build(&args).unwrap_err());

        // Test with unsupported flags:
        let args = vec!["program_name".to_string(), "12".to_string(), "--unknown-flag".to_string()];
        assert!(Config::build(&args).is_ok());
    }
}
