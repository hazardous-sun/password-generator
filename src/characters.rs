use std::arch::x86_64::_mulx_u32;
use rand::Rng; // generates random values

pub struct PreviousCharacters {
    characters: (i8, i8, i8),
}

impl PreviousCharacters {
    pub fn new() -> Self {
        PreviousCharacters {
            characters: (-1, -1, -1)
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
    if last.0 == last.1 {
        repetitions += 1;
    }
    if last.1 == last.2 {
        repetitions += 1;
    }
    if last.0 == last.2 {
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