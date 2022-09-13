#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    // Check for empty list
    if _first_list.is_empty() && _second_list.is_empty() {
        return Comparison::Equal;
    } else {
        if _first_list.is_empty() { return Comparison::Sublist };
        if _second_list.is_empty() { return Comparison::Superlist };
    }

    // Compare equal size lists
    if _first_list.len() == _second_list.len() {
        if _first_list == _second_list { return Comparison::Equal }
        else { return Comparison::Unequal };
    }

    // First list smallest
    if _first_list.len() < _second_list.len() {
        if contains_subarray(_second_list, _first_list) {
            return Comparison::Sublist;
        }
    }

    // Second list smallest
    if _first_list.len() > _second_list.len() {
        if contains_subarray(_first_list, _second_list) {
            return Comparison::Superlist;
        }
    }

    return Comparison::Unequal;
}

fn contains_subarray<T: PartialEq>(array: &[T], subarray: &[T]) -> bool {
    if array.len() < subarray.len() { return false }
    if array.len() == subarray.len() { return array == subarray }

    let mut lbound = 0;
    let mut ubound = subarray.len();
    while ubound <= array.len() {
        if subarray == &array[lbound..ubound] {
            return true;
        }
        lbound = lbound + 1;
        ubound = ubound + 1;
    }
    return false;
}
