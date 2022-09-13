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
        // Slice the second list to compare with the first
        let pos_first = _second_list.iter().position(|x| &x == &_first_list.first().unwrap()).unwrap();
        let pos_last = _second_list.iter().position(|x| &x == &_first_list.last().unwrap()).unwrap();
        let sublist = &_second_list[pos_first..pos_last + 1];

        if _first_list == sublist { return Comparison::Sublist };
    }

    // Second list smallest
    if _first_list.len() > _second_list.len() {
        let pos_first = _first_list.iter().position(|x| &x == &_second_list.first().unwrap()).unwrap();
        let pos_last = _first_list.iter().position(|x| &x == &_second_list.last().unwrap()).unwrap();
        let sublist = &_first_list[pos_first..pos_last + 1];

        if _second_list == sublist { return Comparison::Superlist };
    }

    return Comparison::Unequal;
}
