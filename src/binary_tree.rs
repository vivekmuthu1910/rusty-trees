use crate::trees::Trees;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T>
where
    T: Ord,
{
    value: T,
    left: Link<T>,
    right: Link<T>,
}
impl<T> Node<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        Self {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: T) {
        if self.value > value {
            match self.right {
                None => self.right = Some(Box::new(Node::new(value))),
                Some(ref mut right) => right.insert(value),
            }
        } else if self.value < value {
            match self.left {
                None => self.left = Some(Box::new(Node::new(value))),
                Some(ref mut left) => left.insert(value),
            }
        }
    }
}

#[derive(Debug)]
pub struct BST<T>
where
    T: Ord,
{
    root: Link<T>,
    size: usize,
}

impl<T: Ord> BST<T> {
    pub fn new() -> Self {
        Self {
            root: None,
            size: 0,
        }
    }
}

impl<T> Trees for BST<T>
where
    T: Ord,
{
    type Item = T;
    fn insert(&mut self, value: T) {
        match self.root {
            None => {
                self.root = Some(Box::new(Node::new(value)));
                self.size += 1;
            }
            Some(ref mut root) => root.insert(value),
        }
    }

    fn find_by_value(&self, value: Self::Item) -> Option<&Self::Item> {
        todo!()
    }

    fn find_by_predicate<P>(&self, mut predicate: P) -> Option<&Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        let mut current_node = &self.root;

        loop {
            match current_node {
                Some(ref node) => {
                    if predicate(&node.value) {
                        return current_node.as_ref().map(|node| &node.value);
                    } else {
                    }
                }
                None => return None,
            }
        }
    }

    fn delete_by_value(&mut self, value: Self::Item) -> Option<Self::Item> {
        todo!()
    }
    fn delete_by_predicate<P>(&mut self, predicate: P) -> Option<Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
