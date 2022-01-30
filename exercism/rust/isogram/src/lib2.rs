use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    let mut map: HashMap<String, u8> = HashMap::new();
    for letter in candidate.chars() {
        if letter.is_alphabetic() {
            let count = map.entry(letter.to_lowercase().to_string()).or_insert(0);
            *count += 1;
            if *count > 1 {
                return false;
            }
        }
    }
    true
}
