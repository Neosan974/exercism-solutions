fn is_prime(x: u32) -> bool {
    if x <= 1 {
        return false;
    }

    if x <= 3 {
        return true;
    }

    if x % 2 == 0 || x % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= x {
        if x % i == 0 || x % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}

pub fn nth(n: u32) -> u32 {
    (2..).filter(|&x| is_prime(x)).nth(n as usize).unwrap()
}
