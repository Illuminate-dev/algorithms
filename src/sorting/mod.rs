mod bubble;
mod heapsort;
mod insertion;
mod merge;
mod quicksort;
mod selection;
mod shell;

pub fn is_sorted<T: Ord>(arr: &[T]) -> bool {
    if arr.len() == 0 {
        return true;
    }
    let mut last = &arr[0];
    for i in 1..arr.len() {
        if &arr[i] < last {
            return false;
        }
        last = &arr[i];
    }
    true
}

pub fn have_same_elements<T>(a: &[T], b: &[T]) -> bool
where
    T: std::cmp::PartialOrd + std::cmp::Eq + std::hash::Hash,
{
    use std::collections::HashSet;

    match a.len() == b.len() {
        true => {
            let set_a: HashSet<&T> = a.iter().collect();
            let set_b: HashSet<&T> = b.iter().collect();

            set_a == set_b
        }
        false => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_sorted() {
        let arr = &[1, 2, 4, 4, 5];
        assert!(is_sorted(arr));
        let arr = &[4, 2, 1, 5];
        assert!(!is_sorted(arr));
    }

    #[test]
    fn test_have_same_elements() {
        let arr = &[1, 2, 2, 4];
        let arr2 = &[4, 2, 2, 1];
        assert!(have_same_elements(arr, arr2));

        let arr = &[1, 2, 2, 4, 4];
        assert!(!have_same_elements(arr, arr2));

        let arr = &[1, 4, 4, 2];
        assert!(have_same_elements(arr, arr2));
    }
}
