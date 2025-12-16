/// Smallest Subarray with Sum > X
/// 
/// Returns the length of the smallest subarray. 0 if none.
/// Time Complexity: O(N)
/// Space Complexity: O(1)
pub fn smallest_subarray_gt_x(arr: &[i32], x: i32) -> usize {
    let n = arr.len();
    let mut min_len = n + 1;
    let mut start = 0;
    let mut end = 0;
    let mut curr_sum = 0;
    
    while end < n {
        while curr_sum <= x && end < n {
            curr_sum += arr[end];
            end += 1;
        }
        
        while curr_sum > x && start < n {
            if end - start < min_len {
                min_len = end - start;
            }
            curr_sum -= arr[start];
            start += 1;
        }
    }
    
    if min_len > n { 0 } else { min_len }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_subarray() {
        let arr = vec![1, 4, 45, 6, 0, 19];
        let x = 51;
        // [4, 45, 6] sum=55 > 51. Len 3.
        // [45, 6, 0, 19] sum=70. Len 4.
        // [45, 6] sum=51 (not >).
        assert_eq!(smallest_subarray_gt_x(&arr, x), 3);
    }
}
