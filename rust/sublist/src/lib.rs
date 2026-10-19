#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn are_equal(first: &[i32], second: &[i32]) -> bool {
    if first.len() != second.len() {
        return false;
    }
    for i in 0..first.len() {
        if first[i] != second[i] {
            return false;
        }
    }
    true
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if are_equal(first_list, second_list) {
        return Comparison::Equal;
    } else {
        return Comparison::Unequal;
    }
}
