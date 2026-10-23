fn get_last_sentence(word: &str) -> String {
    format!("And all for the want of a {word}.")
}

fn get_middle_sentence(word: &str, next_word: &str) -> String {
    format!("For want of a {word} the {next_word} was lost.")
}

pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }
    if list.len() == 1 {
        return String::from(get_last_sentence(list[0]));
    }
    let mut proverb = String::new();
    for i in 0..(list.len() - 1) {
        let word = list[i];
        let next_word = list[i + 1];
        proverb += format!("{}\n", get_middle_sentence(word, next_word)).as_str();
    }
    proverb += get_last_sentence(list[0]).as_str();
    proverb
}
