pub fn build_proverb(list: &[&str]) -> String {
    let mut result = Vec::new();
    if list.len() == 0 {
        return String::from("");
    }
    for i in 0..list.len() - 1 {
        let line = format!("For want of a {} the {} was lost.", list[i], list[i + 1]);
        result.push(line);
    }
    let last_line = format!("And all for the want of a {}.", list[0]);
    result.push(last_line);
    result.join("\n")
}
