pub fn reply(message: &str) -> &str {
    let is_a_question = message.trim_end().ends_with("?");
    let is_yelling = message.chars().any(|c| c.is_alphabetic()) && message.to_uppercase() == message;
    let is_silence = message.trim() == "";

    if is_silence {
        return "Fine. Be that way!";
    } else if is_yelling && !is_a_question {
        return "Whoa, chill out!";
    } else if is_a_question && is_yelling {
        return "Calm down, I know what I'm doing!";
    } else if is_a_question && !is_yelling {
        return "Sure.";
    } else {
        return "Whatever.";
    }
}
