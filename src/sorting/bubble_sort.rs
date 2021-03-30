pub fn bubble_sort<T>(arr: &mut [T])
where
    T: Ord,
{
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = [7, 1, 9, 4, 2, 3, 2, 6];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 2, 3, 4, 6, 7, 9]);
    }

    #[test]
    fn test_bubble_sort_descending() {
        let mut arr = [7, 6, 5, 4, 3, 2, 1];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_bubble_sort_ascending() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7];
        bubble_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7]);
    }
}
