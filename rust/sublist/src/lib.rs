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

    let value = first_list.first();
    if value.is_none() {
        // If no value in first, it's always a sublist
        return true;
    }

    let index = second_list.iter().position(|e| e == value.unwrap());
    if index.is_none() {
        // If first item doesn't exist in second list, it cannot be a sublist
        return false;
    } else {
        let second_list_index = index.unwrap();
        for i in 0..first_list.len() {
            if first_list[i] != second_list[second_list_index + i] {
                return false;
            }
        }
    }
    true
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
