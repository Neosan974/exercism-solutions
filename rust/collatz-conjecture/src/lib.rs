fn apply_step(n: u64) -> u64 {
    if n % 2 == 0 {
        return n / 2;
    } else {
        return n * 3 + 1;
    }
}

pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut counter: u64 = 0;
    let mut value = n;
    while value != 1 {
        value = apply_step(value);
        counter += 1;
    }
    Some(counter)
}
