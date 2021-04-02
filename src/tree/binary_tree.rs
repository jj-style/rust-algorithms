#[derive(Debug)]
pub struct BTNode<T: PartialOrd + Copy> {
    data: T,
    left: Option<Box<BTNode<T>>>,
    right: Option<Box<BTNode<T>>>
}

impl<T: PartialOrd + Copy> BTNode<T> {
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
}