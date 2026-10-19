#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn are_equal(first_list: &[i32], second_list: &[i32]) -> bool {
    if first_list.len() != second_list.len() {
        return false;
    }
    for i in 0..first_list.len() {
        if first_list[i] != second_list[i] {
            return false;
        }
    }
    true
}

fn is_sublist(first_list: &[i32], second_list: &[i32]) -> bool {
    if first_list.len() >= second_list.len() {
        return false;
    }

    if first_list.is_empty() {
        return true;
    }

    for i in 0..second_list.len() {
        if second_list[i] == first_list.first().unwrap().to_owned()
            && are_equal(first_list, &second_list[i..(first_list.len() + i)])
        {
            return true;
        }
    }

    false
}

fn is_superlist(first_list: &[i32], second_list: &[i32]) -> bool {
    is_sublist(second_list, first_list)
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if are_equal(first_list, second_list) {
        return Comparison::Equal;
    } else if is_sublist(first_list, second_list) {
        return Comparison::Sublist;
    } else if is_superlist(first_list, second_list) {
        return Comparison::Superlist;
    } else {
        return Comparison::Unequal;
    }
}
