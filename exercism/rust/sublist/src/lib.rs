#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == _second_list {
        return Comparison::Equal;
    }
    if _first_list.is_empty() {
        return Comparison::Sublist;
    }
    if _second_list.is_empty() {
        return Comparison::Superlist;
    }
    let len = _first_list.len();
    let len2 = _second_list.len();
    if len > len2 {
        for i in 0..len {
            if len2 + i > len {
                return Comparison::Unequal;
            }
            if len2 + i <= len && _first_list[i..(len2 + i)].eq(_second_list) {
                return Comparison::Superlist;
            }
        }
    }
    if len < len2 {
        for i in 0..len2 {
            if len + i > len2 {
                return Comparison::Unequal;
            }
            if len + i <= len2 && _second_list[i..(len + i)].eq(_first_list) {
                return Comparison::Sublist;
            }
        }
    }
    Comparison::Unequal
}
