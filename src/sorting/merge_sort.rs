fn _merge<T>(arr: &mut [T], low: usize, mid: usize, high: usize)
where
    T: Ord + Copy,
{
    let mut left = Vec::new();
    let mut right = Vec::new();

    for v in arr.iter().take(mid + 1).skip(low) {
        left.push(*v);
    }
    for v in arr.iter().take(high + 1).skip(mid + 1) {
        right.push(*v);
    }

    let mut l_ptr = 0;
    let mut r_ptr = 0;
    let mut base = low;

    while l_ptr < left.len() && r_ptr < right.len() {
        if left[l_ptr] < right[r_ptr] {
            arr[base] = left[l_ptr];
            l_ptr += 1;
        } else {
            arr[base] = right[r_ptr];
            r_ptr += 1;
        }
        base += 1;
    }

    while l_ptr < left.len() {
        arr[base] = left[l_ptr];
        l_ptr += 1;
        base += 1;
    }

    while r_ptr < right.len() {
        arr[base] = right[r_ptr];
        r_ptr += 1;
        base += 1;
    }
}

fn _merge_sort<T>(arr: &mut [T], low: usize, high: usize)
where
    T: Ord + Copy,
{
    if low < high {
        let mid = low + (high - low) / 2;
        _merge_sort(arr, low, mid);
        _merge_sort(arr, mid + 1, high);
        _merge(arr, low, mid, high);
    }
}

pub fn merge_sort<T>(arr: &mut [T])
where
    T: Ord + Copy,
{
    if arr.len() > 1 {
        _merge_sort(arr, 0, arr.len() - 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut arr = [7, 1, 9, 4, 2, 3, 2, 6];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 2, 3, 4, 6, 7, 9]);
    }

    #[test]
    fn test_merge_sort_descending() {
        let mut arr = [7, 6, 5, 4, 3, 2, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_merge_sort_ascending() {
        let mut arr = [1, 2, 3, 4, 5, 6, 7];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7]);
    }
}
