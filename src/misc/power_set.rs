use std::error::Error;

pub fn power_set<T: Copy>(set: &Vec<T>) -> Result<Vec<Vec<T>>, Box<dyn Error>> {
    let set_length : u32  = set.len() as u32;
    let mut subsets = Vec::with_capacity(i32::pow(2, set_length) as usize);

    for i in 0..i32::pow(2,set_length) {
        let mut subset: Vec<T> = Vec::new();

        // create binary mask, pad with 0s as no dynamic string formatting
        let mut mask = format!("{:b}", i);
        while mask.len() != set.len() {
            mask.insert_str(0, "0");
        }

        // 1 = in the subset, 0 = not in the subset
        for (idx,bit) in mask.chars().enumerate() {
            match bit {
                '0' => (),
                '1' => subset.push(set[idx]),
                e => return Err(format!("expected binary value 1 or 0, got {:?}", e).into())
            }
        }
        subsets.push(subset);
    }
    Ok(subsets)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_set() {
        let source = vec![1,2,3];
        let source_ps = power_set(&source).unwrap();
        assert_eq!(source_ps.len(), 8);
        assert!(source_ps.contains(&vec![]));
        assert!(source_ps.contains(&vec![1]));
        assert!(source_ps.contains(&vec![2]));
        assert!(source_ps.contains(&vec![3]));
        assert!(source_ps.contains(&vec![1,2]));
        assert!(source_ps.contains(&vec![1,3]));
        assert!(source_ps.contains(&vec![2,3]));
        assert!(source_ps.contains(&vec![1,2,3]));
    }
}