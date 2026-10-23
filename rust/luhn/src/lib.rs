use regex::Regex;

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let stripped = code.replace(" ", "");

    // Here checking both length and only numbers in stripped input
    let regex = Regex::new(r"^\d\d+$").unwrap();
    if !regex.is_match(&stripped) {
        return false;
    }

    let mut numbers: Vec<u32> = stripped
        .chars()
        .map(|c| u32::from_str_radix(&c.to_string(), 10).unwrap())
        .collect();
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
