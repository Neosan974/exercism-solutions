#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    // Check input base
    let invalid_bases = [0, 1];
    if invalid_bases.contains(&from_base) {
        return Err(Error::InvalidInputBase);
    }
    // Check output base
    if invalid_bases.contains(&to_base) {
        return Err(Error::InvalidOutputBase);
    }
    // Check input numbers
    for &n in number {
        if n >= from_base {
            return Err(Error::InvalidDigit(n));
        }
    }

    let mut decimal_representation: u32 = 0;

    let nums: Vec<u32> = number.iter().cloned().rev().collect();
    for i in 0..nums.len() {
        decimal_representation += nums[i] * from_base.pow(i as u32);
    }

    let mut result: Vec<u32> = vec![];
    let mut max_power: u32 = 0;
    while to_base.pow(max_power) < decimal_representation {
        max_power += 1;
    }

    let mut i = max_power as i32;
    while decimal_representation != 0 {
        let pow = to_base.pow(i as u32);
        result.push(decimal_representation / pow);
        decimal_representation = decimal_representation % pow;
        i -= 1;
    }

    while i != -1 {
        result.push(0);
        i -= 1;
    }

    while result.first().is_some_and(|&r| r == 0) {
        result.remove(0);
    }

    if result.is_empty() {
        result.push(0);
    }

    Ok(result)
}
