fn heapsort<T: PartialOrd>(arr: &mut [T]) {
    build_heap(arr, arr.len());
    let mut len = arr.len();
    for i in (1..arr.len()).rev() {
        arr.swap(0, i);
        len -= 1;
        heapify(arr, 0, len);
    }
}

fn build_heap<T: PartialOrd>(arr: &mut [T], len: usize) {
    let mut i = len / 2 + 1;
    while i > 0 {
        heapify(arr, i - 1, len);
        i -= 1;
    }
}

fn heapify<T: PartialOrd>(arr: &mut [T], i: usize, len: usize) {
    let left = 2 * i + 1;
    let right = left + 1;
    let mut largest = i;

    if left < len && arr[left] > arr[i] {
        largest = left;
    }

    if right < len && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != i {
        arr.swap(i, largest);
        heapify(arr, largest, len);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heapify() {
        let mut arr = vec![1, 2, 3, 4, 5];
        let len = arr.len();
        heapify(&mut arr, 0, len);
        assert_eq!(arr, vec![3, 2, 1, 4, 5]);
    }

    #[test]
    fn test_build_heap() {
        let mut arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let len = arr.len();
        build_heap(&mut arr, len);
        assert_eq!(arr, vec![10, 9, 7, 8, 5, 6, 3, 1, 4, 2]);
    }

    #[test]
    fn test_heapsort() {
        let mut arr = [51, 23, 5, 1, -1];
        let original = arr.clone();
        heapsort(&mut arr);
        assert!(
            crate::sorting::is_sorted(&arr) && crate::sorting::have_same_elements(&arr, &original)
        );
    }
}
