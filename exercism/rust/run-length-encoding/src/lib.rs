pub fn encode(source: &str) -> String {
    if source.is_empty() {
        return String::new();
    }
    let mut result = String::new();
    let mut count = 1;
    let mut chars = source.chars();
    let mut prev: char = chars.next().unwrap();
    while let Some(c) = chars.next() {
        if c != prev {
            if count > 1 {
                result.push_str(&count.to_string());
            }
            result.push(prev);
            prev = c;
            count = 1;
        } else {
            count += 1;
        }
    }
    if count > 1 {
        result.push_str(&count.to_string());
    }
    result.push(prev);
    result
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let mut count = 0;
    for letter in source.chars() {
        if letter.is_numeric() {
            count = count * 10 + letter.to_digit(10).unwrap();
        } else {
            if count == 0 {
                count = 1;
            }
            for _ in 0..count {
                result.push(letter);
            }
            count = 0;
        }
    }
    result
}
