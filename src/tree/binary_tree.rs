use std::cmp::Ordering::{Equal, Greater, Less};

#[derive(Debug)]
/// Node represents a node in a binary tree.
/// It contains some data and some optional children (left & right only)
pub struct Node<T: Ord + Copy> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

#[derive(Clone, Copy, PartialEq)]
enum Traverse {
    PreOrder,
    InOrder,
    PostOrder,
}

impl<T: Ord + Copy> Node<T> {
    /// Create a Node with the given data. The node will have no children
    pub fn with_data(data: T) -> Self {
        Node {
            data,
            left: None,
            right: None,
        }
    }

    pub fn data(&self) -> T {
        self.data
    }
    pub fn left(&self) -> Option<&Box<Node<T>>> {
        self.left.as_ref()
    }
    pub fn right(&self) -> Option<&Box<Node<T>>> {
        self.right.as_ref()
    }

    /// Insert a piece of data into the binary tree in the correct position
    /// If the data is equal to a node's data it will be placed to the right
    pub fn insert_data(&mut self, data: T) {
        self.insert_node(Box::from(Node::with_data(data)))
    }

    /// Insert a Node into the binary tree in the correct position
    /// If the Node's data is equal to a node's data it will be placed to the right
    pub fn insert_node(&mut self, node: Box<Node<T>>) {
        if node.data < self.data {
            match self.left.as_mut() {
                Some(n) => n.insert_node(node),
                None => self.left = Some(node),
            }
        } else {
            match self.right.as_mut() {
                Some(n) => n.insert_node(node),
                None => self.right = Some(node),
            }
        }
    }

    /// Search for a piece of data in the binary tree
    /// Returns:
    /// - bool - whether the data was found in the tree
    pub fn search(&self, data: T) -> bool {
        match data.cmp(&self.data) {
            Equal => true,
            Less => match self.left.as_ref() {
                Some(n) => n.search(data),
                None => false,
            },
            Greater => match self.right.as_ref() {
                Some(n) => n.search(data),
                None => false,
            },
        }
    }

    /// Recursively traverse a binary tree with a given traversal ordering.
    /// Update the *visited* Vec with the nodes' data in the order they are visited.
    fn _traverse(&self, visited: &mut Vec<T>, traverse_type: Traverse) {
        // TODO - change traverse to return list of nodes not data. Will have to update tree_sort too to check node's data in unit tests
        if traverse_type == Traverse::PreOrder {
            visited.push(self.data);
        }

        match self.left.as_ref() {
            Some(n) => n._traverse(visited, traverse_type),
            None => {}
        }

        if traverse_type == Traverse::InOrder {
            visited.push(self.data);
        }

        match self.right.as_ref() {
            Some(n) => n._traverse(visited, traverse_type),
            None => {}
        }

        if traverse_type == Traverse::PostOrder {
            visited.push(self.data);
        }
    }

    pub fn traverse_in_order(&self) -> Vec<T> {
        let mut visited: Vec<T> = Vec::new();
        self._traverse(&mut visited, Traverse::InOrder);
        visited
    }

    pub fn traverse_pre_order(&self) -> Vec<T> {
        let mut visited: Vec<T> = Vec::new();
        self._traverse(&mut visited, Traverse::PreOrder);
        visited
    }

    pub fn traverse_post_order(&self) -> Vec<T> {
        let mut visited: Vec<T> = Vec::new();
        self._traverse(&mut visited, Traverse::PostOrder);
        visited
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node() {
        let node = Node::with_data(5);
        assert_eq!(node.data(), 5);
        assert_eq!(node.left().is_none(), true);
        assert_eq!(node.right().is_none(), true);
    }

    #[test]
    fn test_data_ordering() {
        let mut node = Node::with_data(5);
        assert_eq!(node.data(), 5);
        assert_eq!(node.left().is_none(), true);
        assert_eq!(node.right().is_none(), true);

        node.insert_data(10);
        assert_eq!(node.left().is_none(), true);
        assert_eq!(node.right().is_some(), true);
        assert_eq!(node.right().unwrap().data(), 10);

        node.insert_data(3);
        assert_eq!(node.left().is_some(), true);
        assert_eq!(node.right().is_some(), true);
        assert_eq!(node.left().unwrap().data(), 3);

        node.insert_data(7);
        assert_eq!(node.right().unwrap().right().is_none(), true);
        assert_eq!(node.right().unwrap().left().is_some(), true);
        assert_eq!(node.right().unwrap().left().unwrap().data(), 7);
    }

    #[test]
    fn test_insert_nodes() {
        let mut node = Node::with_data(5);
        assert_eq!(node.data(), 5);
        assert_eq!(node.left().is_none(), true);
        assert_eq!(node.right().is_none(), true);

        let new_node = Node::with_data(10);
        node.insert_node(Box::from(new_node));
        assert_eq!(node.left().is_none(), true);
        assert_eq!(node.right().is_some(), true);
        assert_eq!(node.right().unwrap().data(), 10);

        let new_node = Node::with_data(3);
        node.insert_node(Box::from(new_node));
        assert_eq!(node.left().is_some(), true);
        assert_eq!(node.right().is_some(), true);
        assert_eq!(node.left().unwrap().data(), 3);

        let new_node = Node::with_data(7);
        node.insert_node(Box::from(new_node));
        assert_eq!(node.right().unwrap().right().is_none(), true);
        assert_eq!(node.right().unwrap().left().is_some(), true);
        assert_eq!(node.right().unwrap().left().unwrap().data(), 7);
    }

    #[test]
    fn test_bt_search() {
        let mut node = Node::with_data(5);
        node.insert_data(10);
        node.insert_data(3);
        node.insert_data(7);

        assert_eq!(node.search(5), true);
        assert_eq!(node.search(10), true);
        assert_eq!(node.search(3), true);
        assert_eq!(node.search(7), true);

        assert_eq!(node.search(1), false);
        assert_eq!(node.search(2), false);
        assert_eq!(node.search(200), false);
        assert_eq!(node.search(29), false);
    }

    #[test]
    fn test_bt_in_order_traversal() {
        let mut node = Node::with_data(5);
        node.insert_data(10);
        node.insert_data(3);
        node.insert_data(7);
        let in_order_data = node.traverse_in_order();
        assert_eq!(in_order_data, [3, 5, 7, 10]);
    }

    #[test]
    fn test_bt_pre_order_traversal() {
        let mut node = Node::with_data(5);
        node.insert_data(10);
        node.insert_data(3);
        node.insert_data(7);
        let pre_order_data = node.traverse_pre_order();
        assert_eq!(pre_order_data, [5, 3, 10, 7]);
    }

    #[test]
    fn test_bt_post_order_traversal() {
        let mut node = Node::with_data(5);
        node.insert_data(10);
        node.insert_data(3);
        node.insert_data(7);
        let post_order_data = node.traverse_post_order();
        assert_eq!(post_order_data, [3, 7, 10, 5]);
    }
}
