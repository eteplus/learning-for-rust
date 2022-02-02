use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut map = HashMap::new();
    let list = words
        .split(|s: char| !s.is_alphabetic() && !s.is_numeric() && s != '\'')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();
    for word in list.iter() {
        let count = map
            .entry(word.trim_matches('\'').to_lowercase())
            .or_insert(0);
        *count += 1;
    }
    map
}
