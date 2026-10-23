pub fn factors(n: u64) -> Vec<u64> {
    let mut value = n;
    let mut result: Vec<u64> = vec![];
    if n == 1 {
        return result;
    }
    let mut i: u64 = 2;
    loop {
        if i == value {
            result.push(value);
            break;
        }
        if (value % i) == 0 {
            result.push(i);
            value /= i;
        } else {
            i += 1;
        }
    }
    result
}
