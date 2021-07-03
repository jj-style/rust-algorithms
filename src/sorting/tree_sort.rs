use crate::tree::Node;

pub fn tree_sort<T: Ord + Copy>(arr: &[T]) -> Vec<T> {
    return if arr.len() < 2 {
        arr.to_vec()
    } else {
        let mut root = Node::with_data(arr[0]);
        for datum in arr.into_iter().skip(1) {
            root.insert_data(*datum);
        }
        root.traverse_in_order()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_sort() {
        let arr = [7, 1, 9, 4, 2, 3, 2, 6];
        let sorted = tree_sort(&arr);
        assert_eq!(sorted, vec![1, 2, 2, 3, 4, 6, 7, 9]);
    }

    #[test]
    fn test_tree_sort_descending() {
        let arr = [7, 6, 5, 4, 3, 2, 1];
        let sorted = tree_sort(&arr);
        assert_eq!(sorted, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_tree_sort_ascending() {
        let arr = [1, 2, 3, 4, 5, 6, 7];
        let sorted = tree_sort(&arr);
        assert_eq!(sorted, vec![1, 2, 3, 4, 5, 6, 7]);
    }
}
