pub fn get_letter_value(c: char) -> u64 {
    if !c.is_ascii_alphabetic() {
        return 0;
    }
    match c {
        'd' | 'g' => 2,
        'b' | 'c' | 'm' | 'p' => 3,
        'f' | 'h' | 'v' | 'w' | 'y' => 4,
        'k' => 5,
        'j' | 'x' => 8,
        'q' | 'z' => 10,
        _ => 1,
    }
}

pub fn score(word: &str) -> u64 {
    word.to_lowercase().chars().map(get_letter_value).sum()
}
