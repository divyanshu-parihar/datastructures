/// Find Largest Sum Contiguous Subarray (Kadane's Algorithm)
/// 
/// Returns the maximum sum.
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn max_subarray_sum(arr: &[i32]) -> i32 {
    if arr.is_empty() {
        return 0;
    }

    let mut max_so_far = arr[0];
    let mut current_max = arr[0];

    for &item in arr.iter().skip(1) {
        current_max = std::cmp::max(item, current_max + item);
        max_so_far = std::cmp::max(max_so_far, current_max);
    }

    max_so_far
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kadane_basic() {
        let arr = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        // Subarray: [4, -1, 2, 1] sum = 6
        assert_eq!(max_subarray_sum(&arr), 6);
    }

    #[test]
    fn test_kadane_all_neg() {
        let arr = vec![-2, -5, -9];
        // Max is -2
        assert_eq!(max_subarray_sum(&arr), -2);
    }
}
