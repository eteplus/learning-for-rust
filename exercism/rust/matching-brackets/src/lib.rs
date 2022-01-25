pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets: Vec<char> = vec![];
    for c in string.chars() {
        match c {
            '[' | '{' | '(' => brackets.push(c),
            // ']' | '}' | ')' => {
            //     let bracket = brackets.pop();
            //     if bracket.is_none() {
            //         return false;
            //     }
            //     let bracket = bracket.unwrap();
            //     let result = match bracket {
            //         '[' => c == ']',
            //         '{' => c == '}',
            //         '(' => c == ')',
            //         _ => false,
            //     };
            //     if !result {
            //         return false;
            //     }
            // }
            ']' => if brackets.pop() != Some('[') { return false; },
            '}' => if brackets.pop() != Some('{') { return false; },
            ')' => if brackets.pop() != Some('(') { return false; },
            _ => (),
        }
    }
    brackets.is_empty()
}
