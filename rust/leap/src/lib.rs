fn is_divisible_by(year: u64, num: u64) -> bool {
    year % num == 0
}

pub fn is_leap_year(year: u64) -> bool {
    is_divisible_by(year, 4) && (!is_divisible_by(year, 100) || is_divisible_by(year, 400))
}
