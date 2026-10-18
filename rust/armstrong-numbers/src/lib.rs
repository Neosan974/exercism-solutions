pub fn is_armstrong_number(num: u32) -> bool {
    let stringified = num.to_string();
    let length = stringified.len() as u32;

    let res = stringified
        .split("")
        .into_iter()
        .map(|x| x.parse::<u32>())
        .map(|x| if x.is_ok() { x.unwrap().pow(length) } else { 0 })
        .reduce(|x, y| x + y)
        .unwrap();
    res == num
}
