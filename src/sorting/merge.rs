/// O(n*lg(n))
/// divides the problem into 2 subproblems of size n/2

pub fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    fn merge<T: Ord + Copy>(arr: &mut [T], arr1: &[T], arr2: &[T]) {
        let (mut i, mut j) = (0, 0);

        let mut idx = 0;

        while i < arr1.len() && j < arr2.len() {
            if arr1[i] < arr2[j] {
                arr[idx] = arr1[i];
                i += 1;
            } else {
                arr[idx] = arr2[j];
                j += 1;
            }
            idx += 1;
        }

        if i < arr1.len() {
            arr[idx..].copy_from_slice(&arr1[i..]);
        }
        if j < arr2.len() {
            arr[idx..].copy_from_slice(&arr2[j..]);
        }
    }

    let mid = arr.len() / 2;
    merge_sort(&mut arr[0..mid]);
    merge_sort(&mut arr[mid..]);
    let mut y = arr.to_vec();
    merge(&mut y[..], &arr[0..mid], &arr[mid..]);
    arr.copy_from_slice(&y[..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut arr = [51, 23, 5, 1, -1];
        let original = arr.clone();
        merge_sort(&mut arr);
        assert!(
            crate::sorting::is_sorted(&arr) && crate::sorting::have_same_elements(&arr, &original)
        );
    }
}
