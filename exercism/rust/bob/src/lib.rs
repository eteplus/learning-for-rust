fn is_question(message: &str) -> bool {
    message.ends_with("?")
}

fn is_yelling(message: &str) -> bool {
    message.chars().any(|c| c.is_ascii_alphabetic()) && message.to_uppercase() == message
}

pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.is_empty() => "Fine. Be that way!",
        m if is_question(m) && is_yelling(m) => "Calm down, I know what I'm doing!",
        m if is_question(m) => "Sure.",
        m if is_yelling(m) => "Whoa, chill out!",
        _ => "Whatever."
    }
}