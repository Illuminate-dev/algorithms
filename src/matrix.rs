use std::ops::Add;
use std::ops::Mul;

fn dot<T: Mul<Output = T> + Add<Output = T> + Default + Copy>(vec1: &[T], vec2: &[T]) -> T {
    assert!(vec1.len() == vec2.len());
    let mut out = T::default();
    for i in 0..vec1.len() {
        out = out + vec1[i] * vec2[i];
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dot_product() {
        let arr1 = [1, 2, 3];
        let arr2 = [7, 9, 11];

        assert_eq!(dot(&arr1, &arr2), 58);
    }
}
