pub fn reply(message: &str) -> &str {
    if message.trim().len() == 0 {
        return "Fine. Be that way!";
    }

    if message.chars().any(|c| c.is_alphabetic()) {
        if message.to_uppercase() == message && message.trim().ends_with("?") {
            return "Calm down, I know what I'm doing!";
        } else if message.trim().ends_with("?") {
            return "Sure.";
        } else if message.to_uppercase() == message {
            return "Whoa, chill out!";
        }
    }
    if message.trim().ends_with("?") {
        return "Sure.";
    }
    return "Whatever.";
}
