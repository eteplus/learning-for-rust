use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut set: HashSet<&'a str> = HashSet::new();
    let mut chars: Vec<char> = word.to_lowercase().chars().collect();
    chars.sort_unstable();
    let string = chars.iter().collect::<String>();
    for item in possible_anagrams {
        let mut item_chars: Vec<char> = item.to_lowercase().chars().collect();
        item_chars.sort_unstable();
        if word.to_lowercase() != item.to_lowercase() && string == item_chars.iter().collect::<String>() {
            set.insert(item);
        }
    }
    set
}
