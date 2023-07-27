/// O(n^2) sorting algorithm
/// traverses array n^2 times, moving higher values up

pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for _ in 1..arr.len() {
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = [51, 23, 5, 1, -1];
        let original = arr.clone();
        bubble_sort(&mut arr);
        assert!(
            crate::sorting::is_sorted(&arr) && crate::sorting::have_same_elements(&arr, &original)
        );
    }
}
