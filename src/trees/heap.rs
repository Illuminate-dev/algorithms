use std::ops::{Index, IndexMut};

struct MaxHeap<T: PartialOrd> {
    data: Vec<T>,
}

impl<T: PartialOrd> MaxHeap<T> {
    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut heap = Self::new();
        for elem in iter {
            heap.push(elem);
        }
        heap
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn heapify(&mut self, index: usize) {
        let left = Self::left(index);
        let right = Self::right(index);

        let mut largest = if left < self.len() && self[left] > self[index] {
            left
        } else {
            index
        };

        if right < self.len() && self[right] > self[largest] {
            largest = right;
        }

        if largest != index {
            self.data.swap(index, largest);
            self.heapify(largest);
        }
    }

    fn push(&mut self, elem: T) {
        self.data.push(elem);
        let mut index = self.len() - 1;
        while index > 0 && self[index] > self[Self::parent(index)] {
            self.data.swap(index, Self::parent(index));
            index = Self::parent(index);
        }
    }

    fn pop_max(&mut self) -> Option<T> {
        if self.len() == 0 {
            return None;
        }
        let len = self.len();
        self.data.swap(0, len - 1);
        let elem = self.data.pop();
        self.heapify(0);
        return elem;
    }

    fn maximum(&self) -> Option<&T> {
        self.data.first()
    }

    fn parent(index: usize) -> usize {
        (index - 1) / 2
    }

    fn left(index: usize) -> usize {
        2 * index + 1
    }

    fn right(index: usize) -> usize {
        2 * index + 2
    }
}

impl<T: PartialOrd> Index<usize> for MaxHeap<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T: PartialOrd> IndexMut<usize> for MaxHeap<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let heap = MaxHeap::<i32>::new();
        assert_eq!(heap.len(), 0);
    }

    #[test]
    fn heapify() {
        let mut heap = MaxHeap::<i32>::new();
        heap.data = vec![1, 2, 3];
        heap.heapify(0);
        assert_eq!(heap.data, vec![3, 2, 1]);
    }

    #[test]
    fn from_iter() {
        let heap = MaxHeap::from_iter(vec![1, 2, 3]);
        assert_eq!(heap.data, vec![3, 1, 2]);
    }

    #[test]
    fn push() {
        let mut heap = MaxHeap::from_iter(vec![1, 2, 3]);
        heap.push(4);
        assert_eq!(heap.data[0], 4);
    }

    #[test]
    fn pop_max() {
        let mut heap = MaxHeap::from_iter(vec![1, 2, 3]);
        heap.pop_max();
        assert_eq!(heap.data, vec![2, 1]);
    }

    #[test]
    fn push_pop() {
        let mut heap = MaxHeap::new();
        heap.push(1);
        heap.push(2);
        heap.push(3);
        heap.push(4);
        heap.push(6);
        assert_eq!(heap.pop_max(), Some(6));
        assert_eq!(heap.pop_max(), Some(4));
        assert_eq!(heap.pop_max(), Some(3));
    }
}
