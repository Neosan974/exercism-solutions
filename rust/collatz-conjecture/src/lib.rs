pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut counter: u64 = 0;
    if n % 2 {
        collatz(n / 2);
    }
    if n == 1 {
        return Some(counter);
    }
    counter += 1;
}
