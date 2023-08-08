use std::ptr::NonNull;

struct BinaryTree<T: PartialOrd> {
    root: Link<T>,
    len: usize,
}

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Self {
            elem,
            left: None,
            right: None,
        }
    }
}

impl<T: PartialOrd> BinaryTree<T> {
    fn new() -> Self {
        Self { root: None, len: 0 }
    }

    fn add(&mut self, elem: T) {
        unsafe {
            let node = NonNull::new_unchecked(Box::into_raw(Box::new(Node::new(elem))));
            let mut current = &mut self.root;
            while let Some(curr) = current {
                if (*curr.as_ptr()).elem > (*node.as_ptr()).elem {
                    current = &mut (*curr.as_ptr()).left;
                } else {
                    current = &mut (*curr.as_ptr()).right;
                }
            }

            *current = Some(node);
            self.len += 1;
        }
    }

    fn search(&self, elem: T) -> bool {
        unsafe {
            let mut current = &self.root;
            while let Some(curr) = current {
                if (*curr.as_ptr()).elem == elem {
                    return true;
                } else if (*curr.as_ptr()).elem > elem {
                    current = &(*curr.as_ptr()).left;
                } else {
                    current = &(*curr.as_ptr()).right;
                }
            }

            false
        }
    }

    fn peek_root(&self) -> Option<&T> {
        unsafe { self.root.map(|node| &(*node.as_ptr()).elem) }
    }

    fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let tree: BinaryTree<i32> = BinaryTree::new();
        assert_eq!(tree.len, 0);
    }

    #[test]
    fn test_add() {
        let mut tree: BinaryTree<i32> = BinaryTree::new();
        tree.add(1);
        tree.add(2);

        assert_eq!(tree.peek_root(), Some(&1));
        assert_eq!(tree.len, 2);
    }

    #[test]
    fn test_search() {
        let mut tree: BinaryTree<i32> = BinaryTree::new();
        for i in 0..10 {
            tree.add(i);
        }

        for i in 0..10 {
            assert_eq!(tree.search(i), true);
        }

        assert_eq!(tree.search(10), false);
        assert_eq!(tree.search(-100), false);
    }
}
