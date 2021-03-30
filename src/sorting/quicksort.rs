use std::cmp::Ordering::{Equal, Greater, Less};

pub fn quicksort<T>(arr: Vec<T>) -> Vec<T>
where
    T: Ord + std::fmt::Debug + Clone,
{
    if arr.len() <= 1 {
        return arr;
    }

    let pivot = arr[0].clone();

    let mut less: Vec<T> = vec![];
    let mut greater: Vec<T> = vec![];
    let mut equal: Vec<T> = vec![];

    for item in arr.into_iter() {
        match item.cmp(&pivot) {
            Less => less.push(item),
            Equal => equal.push(item),
            Greater => greater.push(item),
        }
    }

    let mut less_s = quicksort(less);
    let mut greater_s = quicksort(greater);
    less_s.append(&mut equal);
    less_s.append(&mut greater_s);
    less_s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quicksort() {
        let arr = [7, 1, 9, 4, 2, 3, 2, 6];
        let sorted_arr = quicksort(arr.to_vec());
        assert_eq!(sorted_arr, [1, 2, 2, 3, 4, 6, 7, 9]);
    }

    #[test]
    fn test_quicksort_descending() {
        let arr = [7, 6, 5, 4, 3, 2, 1];
        let sorted_arr = quicksort(Vec::from(arr));
        assert_eq!(sorted_arr, [1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_quicksort_ascending() {
        let arr = [1, 2, 3, 4, 5, 6, 7];
        let sorted_arr = quicksort(Vec::from(arr));
        assert_eq!(sorted_arr, [1, 2, 3, 4, 5, 6, 7]);
    }
}
