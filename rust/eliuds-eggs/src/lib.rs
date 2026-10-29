pub fn egg_count(display_value: u32) -> usize {
    let mut count: usize = 0;
    let mut value = display_value;
    while value != 0 {
        if value & 1 == 1 {
            count += 1;
        }
        value = value >> 1;
    }
    count
}
