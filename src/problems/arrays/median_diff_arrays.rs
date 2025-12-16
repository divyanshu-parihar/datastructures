/// Median of Two Sorted Arrays (Different Size)
/// 
/// Returns the median.
/// Time Complexity: O(log(min(N, M)))
/// Space Complexity: O(1)
pub fn median_diff_arrays(arr1: &[i32], arr2: &[i32]) -> f64 {
    let n = arr1.len();
    let m = arr2.len();
    if n > m {
        return median_diff_arrays(arr2, arr1);
    }
    
    let mut low = 0;
    let mut high = n;
    
    while low <= high {
        let partition_x = (low + high) / 2;
        let partition_y = (n + m + 1) / 2 - partition_x;
        
        let max_left_x = if partition_x == 0 { i32::MIN } else { arr1[partition_x - 1] };
        let min_right_x = if partition_x == n { i32::MAX } else { arr1[partition_x] };
        
        let max_left_y = if partition_y == 0 { i32::MIN } else { arr2[partition_y - 1] };
        let min_right_y = if partition_y == m { i32::MAX } else { arr2[partition_y] };
        
        if max_left_x <= min_right_y && max_left_y <= min_right_x {
            if (n + m) % 2 == 0 {
                return (std::cmp::max(max_left_x, max_left_y) as f64 + std::cmp::min(min_right_x, min_right_y) as f64) / 2.0;
            } else {
                return std::cmp::max(max_left_x, max_left_y) as f64;
            }
        } else if max_left_x > min_right_y {
            high = partition_x - 1;
        } else {
            low = partition_x + 1;
        }
    }
    
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_diff() {
        let arr1 = vec![1, 3];
        let arr2 = vec![2];
        // 1, 2, 3 -> 2
        assert_eq!(median_diff_arrays(&arr1, &arr2), 2.0);
    }
    
    #[test]
    fn test_median_diff_even() {
        let arr1 = vec![1, 2];
        let arr2 = vec![3, 4];
        // 1, 2, 3, 4 -> 2.5
        assert_eq!(median_diff_arrays(&arr1, &arr2), 2.5);
    }
}
