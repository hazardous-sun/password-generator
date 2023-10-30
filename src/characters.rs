use rand::Rng; // generates random values

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
