pub fn bubble_sort<T>(arr: &[T]) -> Vec<T>
where
    T: Ord + Clone,
{
    let mut copy = arr.to_vec();
    for i in 0..copy.len() {
        for j in 0..copy.len() - i - 1 {
            if copy[j] > copy[j + 1] {
                copy.swap(j, j + 1);
            }
        }
    }
    copy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let arr = [7, 1, 9, 4, 2, 3, 2, 6];
        let sorted = bubble_sort(&arr);
        println!("sorted is {:?}", sorted);
        assert_eq!(sorted, vec![1, 2, 2, 3, 4, 6, 7, 9]);
    }

    #[test]
    fn test_bubble_sort_descending() {
        let arr = [7, 6, 5, 4, 3, 2, 1];
        let sorted = bubble_sort(&arr);
        assert_eq!(sorted, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_bubble_sort_ascending() {
        let arr = [1, 2, 3, 4, 5, 6, 7];
        let sorted = bubble_sort(&arr);
        assert_eq!(sorted, vec![1, 2, 3, 4, 5, 6, 7]);
    }
}
