pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() || len == 0 {
        return vec![];
    }
    let mut res: Vec<String> = vec![];
    let mut i: usize = 0;
    while (i + len) <= digits.len() {
        let slice = digits.get(i..(i + len)).unwrap();
        res.push(slice.to_string());
        i += 1;
    }
    res
}
