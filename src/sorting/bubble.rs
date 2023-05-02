pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    for _ in 1..arr.len() {
        for i in 0..arr.len() - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
            }
        }
    }
}
