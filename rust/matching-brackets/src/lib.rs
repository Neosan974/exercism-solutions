pub fn brackets_are_balanced(string: &str) -> bool {
    if string.trim().is_empty() {
        return true;
    }

    let brackets: Vec<char> = string.chars().filter(|char| ["(", ")", "[", "]", "{", "}"].contains(&char.to_string().as_str())).collect();
    if brackets.len() % 2 == 1 {
        return false;
    }

    println!("testing {:?}", brackets);

    let mut pile: Vec<char> = vec![];

    for char in brackets {
        match char.to_string().as_str() {
            "(" | "[" | "{" => {
                pile.push(char);
            }
            ")" if !pile.is_empty() && pile.last().unwrap().to_string().as_str() == "(" => {
                pile.pop();
            }
            "]" if !pile.is_empty() && pile.last().unwrap().to_string().as_str() == "[" => {
                pile.pop();
            }
            "}" if !pile.is_empty() && pile.last().unwrap().to_string().as_str() == "{" => {
                pile.pop();
            }
            _ => {}
        }
    }

    pile.is_empty()
}
