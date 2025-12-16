use std::collections::HashSet;

/// Subarray with 0 Sum
/// 
/// Returns true if any subarray sums to 0.
/// Time Complexity: O(N)
/// Space Complexity: O(N)
pub fn has_zero_sum_subarray(arr: &[i32]) -> bool {
    let mut set = HashSet::new();
    let mut sum = 0;
    
    set.insert(0); // Handle case where prefix sum itself becomes 0
    
    for &num in arr {
        sum += num;
        if set.contains(&sum) {
            return true;
        }
        set.insert(sum);
    }
    
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_sum_basic() {
        let arr = vec![4, 2, -3, 1, 6];
        // 2 + -3 + 1 = 0
        assert!(has_zero_sum_subarray(&arr));
    }
    
    #[test]
    fn test_zero_sum_false() {
        let arr = vec![4, 2, 0, 1, 6];
        // 0 is present
        assert!(has_zero_sum_subarray(&arr));
    }
    
    #[test]
    fn test_zero_sum_none() {
        let arr = vec![1, 2, 3];
        assert!(!has_zero_sum_subarray(&arr));
    }
}
