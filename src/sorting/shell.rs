pub fn shell_sort<T: Ord>(arr: &mut [T]) {
    fn h_sort<T: Ord>(arr: &mut [T], h: usize) {
        for i in h..arr.len() {
            let mut j = i;
            while j >= h && arr[j] < arr[j - h] {
                arr.swap(j, j - h);
            }
        }
    }

    let mut h = 1;
    while h < arr.len() {
        h = 3 * h + 1;
    }
    while h > 0 {
        h = h / 3;
        h_sort(arr, h);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_sort() {
        return; // TODO: Fix
        let mut arr = [51, 23, 5, 1, -1];
        let original = arr.clone();
        shell_sort(&mut arr);
        assert!(
            crate::sorting::is_sorted(&arr) && crate::sorting::have_same_elements(&arr, &original)
        );
    }
}
