use crate::trees::Trees;

type Link<T> = Option<Box<BstNode<T>>>;

#[derive(Debug)]
struct BstNode<T>
where
    T: Ord,
{
    value: T,
    left: Link<T>,
    right: Link<T>,
}
impl<T> BstNode<T>
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

    pub fn insert_2(&mut self, value: T) {}
}

impl<T> Trees for BST<T>
where
    T: Ord,
{
    type Item = T;
    fn insert(&mut self, value: T) -> Result<(), T> {
        match self.root {
            None => {
                self.root = Some(Box::new(BstNode::new(value)));
                self.size += 1;
                Ok(())
            }
            Some(ref mut root) => {
                let mut current_node = root;

                loop {
                    if current_node.value < value {
                        match current_node.right {
                            None => {
                                current_node.right = Some(Box::new(BstNode::new(value)));
                                self.size += 1;
                                return Ok(());
                            }
                            Some(ref mut node) => current_node = node,
                        }
                    } else if current_node.value > value {
                        match current_node.left {
                            None => {
                                current_node.left = Some(Box::new(BstNode::new(value)));
                                self.size += 1;
                                return Ok(());
                            }
                            Some(ref mut node) => current_node = node,
                        }
                    } else {
                        return Err(value);
                    }
                }
            }
        }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn find_by_value(&self, value: Self::Item) -> Option<&Self::Item> {
        let mut current_node = &self.root;

        loop {
            match current_node {
                Some(ref node) => {
                    if node.value == value {
                        return current_node.as_ref().map(|node| &node.value);
                    } else if node.value < value {
                        current_node = &node.right;
                    } else if node.value > value {
                        current_node = &node.left
                    }
                }
                None => return None,
            }
        }
    }

    fn find_by_predicate<P>(&self, predicate: P) -> Option<&Self::Item>
    where
        P: FnMut(&Self::Item) -> bool,
    {
        todo!()
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
    use crate::trees::Trees;

    use super::BST;
    #[test]
    fn create_i32_bst() {
        let mut bst = BST::<i32>::new();
        bst.insert(43).unwrap();
        bst.insert(12).unwrap();
        bst.insert(412).unwrap();
        bst.insert(3).unwrap();

        assert_eq!(bst.len(), 4);
        assert_eq!(bst.find_by_value(412), Some(&412));
        assert_eq!(Err(43), bst.insert(43));
    }
}
