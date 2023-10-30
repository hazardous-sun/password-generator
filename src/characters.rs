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
