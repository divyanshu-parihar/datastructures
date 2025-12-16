use std::collections::HashMap;

/// Check if arr2 is a subset of arr1
/// 
/// Returns true if all elements of arr2 are present in arr1 with at least same frequency.
/// Time Complexity: O(N + M)
/// Space Complexity: O(N)
pub fn is_subset(arr1: &[i32], arr2: &[i32]) -> bool {
    let mut freq_map = HashMap::new();
    
    for &num in arr1 {
        *freq_map.entry(num).or_insert(0) += 1;
    }
    
    for &num in arr2 {
        let count = freq_map.entry(num).or_insert(0);
        if *count == 0 {
            return false;
        }
        *count -= 1;
    }
    
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_subset_true() {
        let arr1 = vec![11, 1, 13, 21, 3, 7];
        let arr2 = vec![11, 3, 7, 1];
        assert!(is_subset(&arr1, &arr2));
    }
    
    #[test]
    fn test_is_subset_false() {
        let arr1 = vec![1, 2, 3];
        let arr2 = vec![1, 2, 4];
        assert!(!is_subset(&arr1, &arr2));
    }
}
