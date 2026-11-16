pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(&[' ', '-'])
        .map(|word| word.replace("_", ""))
        .filter(|word| !word.is_empty())
        .map(|word| {
            let mut letters = word.get(0..1).unwrap().to_string();
            let all_uppercase = word.chars().all(|c| c.is_uppercase());
            let all_lowercase = word.chars().all(|c| c.is_lowercase());
            if !all_uppercase && !all_lowercase {
                let chars: Vec<char> = word.chars().into_iter().collect();
                for i in 1..chars.len() {
                    let char = chars[i];
                    let prev_char = chars[i - 1];
                    if prev_char.is_lowercase() && char.is_uppercase() {
                        letters.push_str(&char.to_string());
                    }
                }
            }
            letters.to_uppercase()
        })
        .collect()
}
