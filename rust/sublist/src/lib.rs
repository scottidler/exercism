#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn find<T: PartialEq>(haystack: &[T], needle: &[T]) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    if a.len() == 0 && b.len() > 0 {
        Comparison::Sublist
    } else if a.len() > 0 && b.len() == 0 {
        Comparison::Superlist
    } else if a == b {
        Comparison::Equal
    } else if find(a, b) {
        Comparison::Superlist
    } else if find(b, a) {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}
