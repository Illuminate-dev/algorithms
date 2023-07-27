/// O(n*log(n)) average, O(n^2) worst case
pub fn _quicksort<T: Ord>(arr: &mut [T], lo: isize, hi: isize) {
    if lo < hi {
        let q = partition(arr, lo, hi);
        _quicksort(arr, lo, q - 1);
        _quicksort(arr, q + 1, hi);
    }
}

pub fn partition<T: Ord>(arr: &mut [T], lo: isize, hi: isize) -> isize {
    let pivot = hi;

    let mut i = lo;

    for j in lo..hi {
        if arr[pivot as usize] >= arr[j as usize] {
            arr.swap(i as usize, j as usize);
            i += 1;
        }
    }

    arr.swap(i as usize, pivot as usize);
    i 
}

pub fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.len() > 0 {
        _quicksort(arr, 0, arr.len() as isize-1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_quicksort() {
        let mut arr = [51, 23, 5, 1, -1];
        let original = arr.clone();
        quicksort(&mut arr);
        assert!(
            crate::sorting::is_sorted(&arr) && crate::sorting::have_same_elements(&arr, &original)
        );
    }
}
