pub fn binary_search<T: Ord + std::fmt::Debug>(list: &[T], item: T) -> i32 {
    let mut low = 0;
    let mut high = list.len() as i32 - 1;

    while low <= high {
        let mid = (high + low) / 2;
        if list[mid as usize] < item {
            low = mid + 1;
        } else if list[mid as usize] > item {
            high = mid - 1;
        } else {
            return mid;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&arr, 1), 0);
        assert_eq!(binary_search(&arr, 5), 4);
        assert_eq!(binary_search(&arr, 3), 2);
        assert_eq!(binary_search(&arr, 2), 1);
    }
}
