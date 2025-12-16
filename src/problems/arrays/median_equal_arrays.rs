/// Median of Two Sorted Arrays (Equal Size)
/// 
/// Returns the median.
/// Time Complexity: O(log N)
/// Space Complexity: O(1)
pub fn median_equal_arrays(arr1: &[i32], arr2: &[i32]) -> f64 {
    let n = arr1.len();
    if n == 0 { return 0.0; }
    
    // Recursive approach
    get_median(arr1, arr2, n)
}

fn get_median(arr1: &[i32], arr2: &[i32], n: usize) -> f64 {
    if n <= 0 { return 0.0; }
    if n == 1 { return (arr1[0] + arr2[0]) as f64 / 2.0; }
    if n == 2 {
        return (std::cmp::max(arr1[0], arr2[0]) as f64 + std::cmp::min(arr1[1], arr2[1]) as f64) / 2.0;
    }
    
    let m1 = median(arr1, n);
    let m2 = median(arr2, n);
    
    if (m1 - m2).abs() < f64::EPSILON {
        return m1;
    }
    
    if m1 < m2 {
        if n % 2 == 0 {
            get_median(&arr1[n/2 - 1..], &arr2[..n/2 + 1], n/2 + 1)
        } else {
            get_median(&arr1[n/2..], &arr2[..n/2 + 1], n/2 + 1)
        }
    } else {
        if n % 2 == 0 {
            get_median(&arr1[..n/2 + 1], &arr2[n/2 - 1..], n/2 + 1)
        } else {
            get_median(&arr1[..n/2 + 1], &arr2[n/2..], n/2 + 1)
        }
    }
}

fn median(arr: &[i32], n: usize) -> f64 {
    if n % 2 == 0 {
        (arr[n/2] + arr[n/2 - 1]) as f64 / 2.0
    } else {
        arr[n/2] as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_equal() {
        let arr1 = vec![1, 12, 15, 26, 38];
        let arr2 = vec![2, 13, 17, 30, 45];
        // Merged: 1, 2, 12, 13, 15, 17, 26, 30, 38, 45
        // Median (15+17)/2 = 16
        assert_eq!(median_equal_arrays(&arr1, &arr2), 16.0);
    }
}
