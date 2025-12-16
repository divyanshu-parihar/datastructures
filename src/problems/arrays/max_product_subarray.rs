/// Maximum Product Subarray
/// 
/// Returns the maximum product of a contiguous subarray.
/// Time Complexity: O(N)
/// Space Complexity: O(1)
pub fn max_product(nums: &[i32]) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    
    let mut max_so_far = nums[0];
    let mut min_so_far = nums[0];
    let mut result = max_so_far;
    
    for &num in nums.iter().skip(1) {
        let curr = num;
        let temp_max = std::cmp::max(curr, std::cmp::max(max_so_far * curr, min_so_far * curr));
        min_so_far = std::cmp::min(curr, std::cmp::min(max_so_far * curr, min_so_far * curr));
        max_so_far = temp_max;
        
        result = std::cmp::max(result, max_so_far);
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_product_basic() {
        let nums = vec![2, 3, -2, 4];
        // 2*3 = 6
        assert_eq!(max_product(&nums), 6);
    }

    #[test]
    fn test_max_product_neg() {
        let nums = vec![-2, 0, -1];
        // 0
        assert_eq!(max_product(&nums), 0);
    }
    
    #[test]
    fn test_max_product_double_neg() {
        let nums = vec![-2, 3, -4];
        // -2 * 3 * -4 = 24
        assert_eq!(max_product(&nums), 24);
    }
}
