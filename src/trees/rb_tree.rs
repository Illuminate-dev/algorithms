use std::ptr::NonNull;

struct RedBlackTree<T: PartialOrd> {
    root: Link<T>,
    len: usize,
}

type Link<T> = Option<NonNull<Node<T>>>;

#[derive(PartialEq)]
enum Color {
    Red,
    Black,
}

struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>,
    color: Color,
    parent: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Self {
            elem,
            left: None,
            right: None,
            color: Color::Red,
            parent: None,
        }
    }
}

impl<T: PartialOrd> RedBlackTree<T> {
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    fn left_rotate(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            let y = (*node.as_ptr()).right.unwrap();
            (*node.as_ptr()).right = (*y.as_ptr()).left;

            if let Some(left) = (*y.as_ptr()).left {
                (*left.as_ptr()).parent = Some(node);
            }

            (*y.as_ptr()).parent = (*node.as_ptr()).parent;

            if (*node.as_ptr()).parent.is_none() {
                self.root = Some(y);
            } else if Some(node) == (*(*node.as_ptr()).parent.unwrap().as_ptr()).left {
                (*(*node.as_ptr()).parent.unwrap().as_ptr()).left = Some(y);
            } else {
                (*(*node.as_ptr()).parent.unwrap().as_ptr()).right = Some(y);
            }

            (*y.as_ptr()).left = Some(node);

            (*node.as_ptr()).parent = Some(y);
        }
    }

    fn right_rotate(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            let y = (*node.as_ptr()).left.unwrap();
            (*node.as_ptr()).left = (*y.as_ptr()).right;

            if let Some(right) = (*y.as_ptr()).right {
                (*right.as_ptr()).parent = Some(node);
            }

            (*y.as_ptr()).parent = (*node.as_ptr()).parent;

            if (*node.as_ptr()).parent.is_none() {
                self.root = Some(y);
            } else if Some(node) == (*(*node.as_ptr()).parent.unwrap().as_ptr()).left {
                (*(*node.as_ptr()).parent.unwrap().as_ptr()).left = Some(y);
            } else {
                (*(*node.as_ptr()).parent.unwrap().as_ptr()).right = Some(y);
            }

            (*y.as_ptr()).right = Some(node);

            (*node.as_ptr()).parent = Some(y);
        }
    }

    fn insert(&mut self, elem: T) {
        unsafe {
            let mut x = self.root;
            let mut y = None;

            while let Some(curr) = x {
                y = x;
                if (*curr.as_ptr()).elem > elem {
                    x = (*curr.as_ptr()).left;
                } else {
                    x = (*curr.as_ptr()).right;
                }
            }

            let node = NonNull::new_unchecked(Box::into_raw(Box::new(Node::new(elem))));

            (*node.as_ptr()).parent = y;
            if y.is_none() {
                self.root = Some(node);
            } else if (*y.unwrap().as_ptr()).elem < (*node.as_ptr()).elem {
                (*y.unwrap().as_ptr()).right = Some(node);
            } else {
                (*y.unwrap().as_ptr()).left = Some(node);
            }

            self.len += 1;
            self.insert_fixup(node);
        }
    }

    fn insert_fixup(&mut self, mut node: NonNull<Node<T>>) {
        unsafe {
            while (*node.as_ptr()).parent.is_some()
                && (*(*node.as_ptr()).parent.unwrap().as_ptr()).color == Color::Red
            {
                let grandparent = (*(*node.as_ptr()).parent.unwrap().as_ptr()).parent.unwrap();
                let parent = (*node.as_ptr()).parent.unwrap();

                if (*node.as_ptr()).parent == (*grandparent.as_ptr()).left {
                    let y = (*grandparent.as_ptr()).right;

                    if y.is_some() && (*y.unwrap().as_ptr()).color == Color::Red {
                        (*parent.as_ptr()).color = Color::Black;
                        (*y.unwrap().as_ptr()).color = Color::Black;
                        (*grandparent.as_ptr()).color = Color::Red;
                        node = grandparent;
                    } else {
                        if Some(node) == (*parent.as_ptr()).right {
                            node = parent;
                            self.left_rotate(node);
                        }
                        (*parent.as_ptr()).color = Color::Black;
                        (*grandparent.as_ptr()).color = Color::Red;
                        self.right_rotate(grandparent);
                    }
                } else {
                    let y = (*grandparent.as_ptr()).left;
                    if y.is_some() && (*y.unwrap().as_ptr()).color == Color::Red {
                        (*parent.as_ptr()).color = Color::Black;
                        (*y.unwrap().as_ptr()).color = Color::Black;
                        (*grandparent.as_ptr()).color = Color::Red;
                        node = grandparent;
                    } else {
                        if Some(node) == (*parent.as_ptr()).left {
                            node = parent;
                            self.right_rotate(node);
                        }
                        (*parent.as_ptr()).color = Color::Black;
                        (*grandparent.as_ptr()).color = Color::Red;
                        self.left_rotate(grandparent);
                    }
                }
            }
            (*self.root.unwrap().as_ptr()).color = Color::Black;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        let tree = RedBlackTree::<i32>::new();
        assert_eq!(tree.len, 0);
    }

    #[test]
    fn insert() {
        let mut tree = RedBlackTree::<i32>::new();
        tree.insert(1);
        tree.insert(2);
        tree.insert(3);
        assert_eq!(tree.len, 3);
        unsafe {
            assert_eq!((*tree.root.unwrap().as_ptr()).elem, 2);
            assert_eq!(
                (*(*tree.root.unwrap().as_ptr()).left.unwrap().as_ptr()).elem,
                1
            );
            assert_eq!(
                (*(*tree.root.unwrap().as_ptr()).right.unwrap().as_ptr()).elem,
                3
            );
        }
    }

    #[test]
    fn search() {
        let mut tree = RedBlackTree::<i32>::new();
        tree.insert(1);
        tree.insert(2);
        tree.insert(3);
        tree.insert(4);
        tree.insert(5);
        tree.insert(6);
        assert_eq!(tree.search(1), true);
        assert_eq!(tree.search(2), true);
        assert_eq!(tree.search(3), true);
        assert_eq!(tree.search(4), true);
        assert_eq!(tree.search(5), true);
        assert_eq!(tree.search(6), true);
        assert_eq!(tree.search(7), false);
        assert_eq!(tree.search(8), false);
        assert_eq!(tree.search(9), false);
    }
}
