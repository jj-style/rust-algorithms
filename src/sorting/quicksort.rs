use std::cmp::Ordering::{Less, Equal, Greater};

pub fn quicksort<T>(arr: &mut [T]) -> &[T] where T: Ord + std::fmt::Debug {
    
    if arr.len() > 1 {
        let mut less = Vec::new();
        let mut greater = Vec::new();
        let mut eq = Vec::new();

        let pivot = &arr[0];

        for i in arr.iter() {
            match i.cmp(pivot) {
                Equal => eq.push(i),
                Less => less.push(i),
                Greater => greater.push(i)
            };
        }

        println!("pivot: {:?}", pivot);
        println!("less: {:?}", less);
        println!("eq: {:?}", eq);
        println!("more: {:?}", greater);

        let sorted_less = quicksort(&mut less);
        let sorted_greater = quicksort(&mut greater);

        let mut sorted: Vec<&T> = Vec::new();
        sorted.extend(sorted_less);
        sorted.extend(eq);
        sorted.extend(sorted_greater);
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quicksort() {
        let mut arr = [7,1,9,4,2,3,2,6];
        quicksort(&mut arr);
        assert_eq!(arr, [1,2,2,3,4,6,7,9]);
    }
}