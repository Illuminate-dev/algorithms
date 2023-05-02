/// O(n^2)

pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() - 1 {
        let mut min = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min] {
                min = j;
            }
        }
        arr.swap(i, min);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection() {
        let mut arr = [51, 23, 5, 1, -1];
        let original = arr.clone();
        selection_sort(&mut arr);
        assert!(
            crate::sorting::is_sorted(&arr) && crate::sorting::have_same_elements(&arr, &original)
        );
    }
}
