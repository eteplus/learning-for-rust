use std::collections::HashMap;

/// Compute the Scrabble score for a word.
fn get_letter() -> HashMap<char, u64> {
    HashMap::from_iter([
        ('d', 2),
        ('g', 2),
        ('b', 3),
        ('c', 3),
        ('m', 3),
        ('p', 3),
        ('f', 4),
        ('h', 4),
        ('v', 4),
        ('w', 4),
        ('y', 4),
        ('k', 5),
        ('j', 8),
        ('x', 8),
        ('q', 10),
        ('z', 10),
    ])
}

pub fn score(word: &str) -> u64 {
    let letter = get_letter();
    let mut sum = 0;
    for c in word.to_lowercase().chars() {
        if c.is_ascii_alphabetic() {
            sum += letter.get(&c).unwrap_or(&1);
        }
    }
    sum
}