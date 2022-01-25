pub fn brackets_are_balanced(string: &str) -> bool {
    let mut vec = vec![];
    for c in string.chars() {
        match c {
            '[' | '{' | '(' => {
                vec.push(c);
            }
            ']' | '}' | ')' => {
                let bracket = vec.pop();
                if bracket.is_none() {
                    return false;
                }
                let bracket = bracket.unwrap();
                let result = match bracket {
                    '[' => c == ']',
                    '{' => c == '}',
                    '(' => c == ')',
                    _ => false,
                };
                if !result {
                    return false;
                }
            }
            _ => (),
        }
    }
    if vec.is_empty() {
        return true;
    }
    false
}
