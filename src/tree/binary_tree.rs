use std::cmp::Ordering::{Equal, Greater, Less};

#[derive(Debug)]
pub struct BTNode<T: PartialOrd + Ord + Copy> {
    data: T,
    left: Option<Box<BTNode<T>>>,
    right: Option<Box<BTNode<T>>>
}

impl<T: PartialOrd + Ord + Copy> BTNode<T> {
    pub fn new(data: T) -> Self {
        BTNode {
            data,
            left: None,
            right: None
        }
    }
    pub fn get_data(&self) -> T {
        self.data
    }
    pub fn left(&self) -> Option<&Box<BTNode<T>>> {
        self.left.as_ref()
    }
    pub fn right(&self) -> Option<&Box<BTNode<T>>> {
        self.right.as_ref()
    }
    pub fn insert_node(&mut self, data: T) {
        if data < self.data {
            match self.left.as_mut() {
                Some(n) => n.insert_node(data),
                None => self.left = Some(Box::from(BTNode::new(data)))
            }
        } else {
            match self.right.as_mut() {
                Some(n) => n.insert_node(data),
                None => self.right = Some(Box::from(BTNode::new(data)))
            }
        }
    }
    pub fn search(&self, data: T) -> bool {
        return match data.cmp(&self.data) {
            Equal => true,
            Less => match self.left.as_ref() {
                Some(n) => n.search(data),
                None => false
            },
            Greater => match self.right.as_ref() {
                Some(n) => n.search(data),
                None => false
            }
        }
    }

    // TODO: traversal's, in-order, pre-order, post-order
    // Tree-sort -> fn(Vec<T>) -> build binary tree -> in-order traversal -> sorted!
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btnode() {
        let node = BTNode::new(5);
        assert_eq!(node.get_data(), 5);
        assert_eq!(node.left().is_none(), true);
        assert_eq!(node.right().is_none(), true);
    }

    #[test]
    fn test_btnode_ordering() {
        let mut node = BTNode::new(5);
        assert_eq!(node.get_data(), 5);
        assert_eq!(node.left().is_none(), true);
        assert_eq!(node.right().is_none(), true);

        node.insert_node(10);
        assert_eq!(node.left().is_none(), true);
        assert_eq!(node.right().is_some(), true);
        assert_eq!(node.right().unwrap().get_data(), 10);

        node.insert_node(3);
        assert_eq!(node.left().is_some(), true);
        assert_eq!(node.right().is_some(), true);
        assert_eq!(node.left().unwrap().get_data(), 3);

        node.insert_node(7);
        assert_eq!(node.right().unwrap().right().is_none(), true);
        assert_eq!(node.right().unwrap().left().is_some(), true);
        assert_eq!(node.right().unwrap().left().unwrap().get_data(), 7);
    }

    #[test]
    fn test_bt_search() {
        let mut node = BTNode::new(5);
        node.insert_node(10);
        node.insert_node(3);
        node.insert_node(7);

        assert_eq!(node.search(5), true);
        assert_eq!(node.search(10), true);
        assert_eq!(node.search(3), true);
        assert_eq!(node.search(7), true);

        assert_eq!(node.search(1), false);
        assert_eq!(node.search(2), false);
        assert_eq!(node.search(200), false);
        assert_eq!(node.search(29), false);
    }
}