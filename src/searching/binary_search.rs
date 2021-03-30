use std::cmp::Ordering;

fn _binary_search<T: Ord>(v: &Vec<T>, l: isize, h: isize, target: T) -> Option<usize> {
    return if l > h {
        None
    } else {
        let m = (l + h) / 2;
        match v[m as usize].cmp(&target) {
            Ordering::Less => _binary_search(v, m + 1, h, target),
            Ordering::Equal => Some(m as usize),
            Ordering::Greater => _binary_search(v, l, m - 1, target),
        }
    };
}

pub fn binary_search<T: Ord>(v: &Vec<T>, target: T) -> Option<usize> {
    _binary_search(v, 0, (v.len() - 1) as isize, target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_edge_cases_in_vec() {
        let v = vec![1,2,3,4,5];
        assert_eq!(binary_search(&v, 2).unwrap(), 1);
        assert_eq!(binary_search(&v, 3).unwrap(), 2);
        assert_eq!(binary_search(&v, 4).unwrap(), 3);
    }

    #[test]
    fn test_edge_cases_in_vec() {
        let v = vec![1,2,3,4,5];
        assert_eq!(binary_search(&v, 1).unwrap(), 0);
        assert_eq!(binary_search(&v, 5).unwrap(), 4);
    }

    #[test]
    fn test_edge_cases_out_vec() {
        let v = vec![1,2,3,4,5];
        assert_eq!(binary_search(&v, 0).is_none(), true);
        assert_eq!(binary_search(&v, 6).is_none(), true);
    }

    #[test]
    fn test_non_edge_cases_out_vec() {
        let v = vec![1,2,3,4,5];
        assert_eq!(binary_search(&v, -20).is_none(), true);
        assert_eq!(binary_search(&v, 20).is_none(), true);
    }
}