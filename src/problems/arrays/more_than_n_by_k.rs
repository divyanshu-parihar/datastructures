use std::collections::HashMap;

/// Find Elements appearing more than n/k times
/// 
/// Returns a vector of such elements.
/// Time Complexity: O(N)
/// Space Complexity: O(N)
pub fn count_more_than_n_by_k(nums: &[i32], k: usize) -> Vec<i32> {
    if k == 0 || nums.is_empty() {
        return vec![];
    }
    
    let n = nums.len();
    let threshold = n / k;
    let mut counts = HashMap::new();
    
    for &num in nums {
        *counts.entry(num).or_insert(0) += 1;
    }
    
    let mut res = Vec::new();
    for (num, count) in counts {
        if count > threshold {
            res.push(num);
        }
    }
    
    res.sort(); // Optional, for consistent testing
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_more_than_n_by_k() {
        let nums = vec![3, 1, 2, 2, 1, 2, 3, 3];
        let k = 4;
        // n = 8, n/k = 2.
        // 3 appears 3 times (>2).
        // 2 appears 3 times (>2).
        // 1 appears 2 times (not >2).
        assert_eq!(count_more_than_n_by_k(&nums, k), vec![2, 3]);
    }
}
