use crate::tree::BTNode;

pub fn tree_sort<T: Ord + Copy>(v: Vec<T>) -> Vec<T> {
    return if v.len() < 2 {
        v
    } else {
        let mut root = BTNode::new(v[0]);
        for datum in v.into_iter().skip(1) {
            root.insert_node(datum);
        }
        root.traverse_in_order()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_sort() {
        let arr = vec![7, 1, 9, 4, 2, 3, 2, 6];
        let sorted = tree_sort(arr);
        assert_eq!(sorted, [1, 2, 2, 3, 4, 6, 7, 9]);
    }

    #[test]
    fn test_tree_sort_descending() {
        let arr = vec![7, 6, 5, 4, 3, 2, 1];
        let sorted = tree_sort(arr);
        assert_eq!(sorted, [1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_tree_sort_ascending() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7];
        let sorted = tree_sort(arr);
        assert_eq!(sorted, [1, 2, 3, 4, 5, 6, 7]);
    }
}
