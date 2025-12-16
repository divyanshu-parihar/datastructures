/// Minimum Swaps to Group Elements <= K
/// 
/// Returns min swaps required.
/// Time Complexity: O(N)
/// Space Complexity: O(1)
pub fn min_swaps(arr: &[i32], k: i32) -> usize {
    let n = arr.len();
    let mut count = 0;
    
    for &num in arr {
        if num <= k {
            count += 1;
        }
    }
    
    let mut bad = 0;
    for i in 0..count {
        if arr[i] > k {
            bad += 1;
        }
    }
    
    let mut ans = bad;
    for i in 0..n - count {
        if arr[i] > k {
            bad -= 1;
        }
        if arr[i + count] > k {
            bad += 1;
        }
        ans = std::cmp::min(ans, bad);
    }
    
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_swaps() {
        let arr = vec![2, 1, 5, 6, 3];
        let k = 3;
        // count = 3 (2, 1, 3). Window size 3.
        // [2, 1, 5] -> bad 1 (5)
        // [1, 5, 6] -> bad 2 (5, 6)
        // [5, 6, 3] -> bad 2 (5, 6)
        // Min bad is 1.
        assert_eq!(min_swaps(&arr, k), 1);
    }
}
