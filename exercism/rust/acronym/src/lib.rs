pub fn abbreviate(phrase: &str) -> String {
    if phrase.is_empty() {
        return "".to_string();
    }
    let mut letters = Vec::new();
    let mut first = true;
    let mut prev = false;
    for c in phrase.chars() {
        if c.is_ascii_whitespace() || c == '-' || c == '_' {
            first = true;
            continue;
        }

        if first || (!prev && c.is_ascii_uppercase()) {
            letters.push(c.to_ascii_uppercase());
        }

        prev = first || c.is_ascii_uppercase();
        first = false;
    }
    letters.into_iter().collect()
}
