use std::rc::Rc;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem: elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

pub struct Iter<'a, T: 'a> {
    next: Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();

        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn test_creation() {
        let list: List<()> = List::new();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn test_prepend() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        assert_eq!(list.head(), Some(&3));
        assert_eq!(list.tail().head(), Some(&2));
    }

    #[test]
    fn test_iter() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();

        for i in (1..4).rev() {
            assert_eq!(iter.next(), Some(&i));
        }
    }

    #[test]
    fn test_multiple_access() {
        let list = List::new().prepend(1).prepend(2).prepend(3);

        let list2 = list.tail().prepend(5);

        assert_eq!(list.head(), Some(&3));
        assert_eq!(list2.head(), Some(&5));
        assert_eq!(list2.tail().head(), Some(&2));
    }
}
