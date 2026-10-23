/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let stripped = code.replace(" ", "");

    if stripped.len() <= 1 {
        return false;
    }

    let mut chars = stripped.chars();
    if chars.any(|char| !char.is_numeric()) {
        return false;
    }

    let mut numbers: Vec<u32> = vec![];
    for c in stripped.chars() {
        numbers.push(u32::from_str_radix(&c.to_string(), 10).unwrap());
    }
    numbers.reverse();
    for i in 0..numbers.len() {
        if i % 2 == 0 {
            continue;
        }
        let doubled = numbers[i] * 2;
        numbers[i] = if doubled > 9 { doubled - 9 } else { doubled };
    }
    numbers.reverse();
    let sum: u32 = numbers.iter().sum();
    sum % 10 == 0
}
