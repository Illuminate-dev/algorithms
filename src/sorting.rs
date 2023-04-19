pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for _ in 1..arr.len() {
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
            }
        }
    }
}

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
    fn test_insertion_sort() {
        let mut arr = [5, 32, 56, 1, 3, -5];
        insertion_sort(&mut arr);

        assert_eq!(arr, [-5, 1, 3, 5, 32, 56]);
    }

    #[test]
    fn test_merge_sort() {
        let mut arr = [5, 32, 56, 1, 3, -5];
        merge_sort(&mut arr);

        assert_eq!(arr, [-5, 1, 3, 5, 32, 56]);
    }

    #[test]
    fn test_bubble_sort() {
        let mut arr = [5, 32, 56, 1, 3, -5];
        bubble_sort(&mut arr);

        assert_eq!(arr, [-5, 1, 3, 5, 32, 56]);
    }

    #[test]
    fn test_selection_sort() {
        let mut arr = [5, 32, 56, 1, 3, -5];
        bubble_sort(&mut arr);

        assert_eq!(arr, [-5, 1, 3, 5, 32, 56]);
    }
}
