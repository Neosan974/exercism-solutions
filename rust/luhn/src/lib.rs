/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    println!("validating {code}");
    let stripped = code.replace(" ", "");
    println!("stripped {stripped}");

    if stripped.len() <= 1 {
        println!("invalid because too short");
        return false;
    }

    let mut chars = stripped.chars();
    println!("characters {:?}", chars);
    if chars.any(|char| !char.is_numeric()) {
        println!("contains invalid characters");
        return false;
    }

    // let mut numbers: Vec<u32> = chars.map(|char| char.to_digit(10).unwrap()).collect();
    let mut numbers: Vec<u32> = vec![];
    for c in chars.as_str().split("") {
        println!("adding character {c} which is number {}", 0);
        numbers.push(0);
    }
    println!("numbers {:?}", numbers);
    numbers.reverse();
    for i in 0..numbers.len() {
        if i % 2 == 0 {
            continue;
        }
        let doubled = numbers[i] * 2;
        numbers[i] = if doubled > 9 { doubled - 9 } else { doubled };
    }
    numbers.reverse();
    println!("doubled {:?}", numbers);
    let sum: u32 = numbers.iter().sum();
    println!(
        "sum: {sum}, {}",
        if sum % 10 == 0 { "valid" } else { "invalid" }
    );
    sum % 10 == 0
}
