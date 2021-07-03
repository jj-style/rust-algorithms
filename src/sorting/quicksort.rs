use std::cmp::Ordering::{Equal, Greater, Less};

fn _quicksort<T>(arr: Vec<T>) -> Vec<T>
where
    T: Ord + Clone,
{
    if arr.len() <= 1 {
        return arr;
    }

    let pivot = arr[0].clone();
    let max_len = arr.len();

    let mut less: Vec<T> = Vec::new();
    let mut greater: Vec<T> = Vec::new();
    let mut equal: Vec<T> = Vec::new();

    for item in arr.into_iter() {
        match item.cmp(&pivot) {
            Less => less.push(item),
            Equal => equal.push(item),
            Greater => greater.push(item),
        }
    }

    let mut sorted = Vec::with_capacity(max_len);
    let mut less_sorted = _quicksort(less);
    let mut greater_sorted = _quicksort(greater);

    sorted.append(&mut less_sorted);
    sorted.append(&mut equal);
    sorted.append(&mut greater_sorted);
    sorted
}

pub fn quicksort<T>(arr: &[T]) -> Vec<T>
where
    T: Ord + Clone,
{
    _quicksort(arr.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quicksort() {
        let arr = [7, 1, 9, 4, 2, 3, 2, 6];
        let sorted_arr = quicksort(&arr);
        assert_eq!(sorted_arr, vec![1, 2, 2, 3, 4, 6, 7, 9]);
    }

    #[test]
    fn test_quicksort_descending() {
        let arr = [7, 6, 5, 4, 3, 2, 1];
        let sorted_arr = quicksort(&arr);
        assert_eq!(sorted_arr, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_quicksort_ascending() {
        let arr = [1, 2, 3, 4, 5, 6, 7];
        let sorted_arr = quicksort(&arr);
        assert_eq!(sorted_arr, vec![1, 2, 3, 4, 5, 6, 7]);
    }
}
