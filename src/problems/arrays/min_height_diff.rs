/// Minimize the Maximum Difference Between Heights
/// 
/// Given heights and k, modify each height by +k or -k.
/// Minimize the difference between the largest and smallest resulting height.
/// Note: Resulting height cannot be negative.
pub fn min_height_diff(arr: &mut [i32], k: i32) -> i32 {
    let n = arr.len();
    if n <= 1 {
        return 0;
    }

    arr.sort();
    
    let mut ans = arr[n-1] - arr[0];
    
    let smallest = arr[0] + k;
    let largest = arr[n-1] - k;
    
    for i in 0..n-1 {
        let min_h = std::cmp::min(smallest, arr[i+1] - k);
        let max_h = std::cmp::max(largest, arr[i] + k);
        
        if min_h < 0 {
            continue;
        }
        
        ans = std::cmp::min(ans, max_h - min_h);
    }
    
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_height_diff() {
        let mut arr = vec![1, 5, 8, 10];
        let k = 2;
        // 1+2=3, 5-2=3, 8-2=6, 10-2=8 => 3, 3, 6, 8. Diff = 5.
        // Or 1+2=3, 5+2=7, 8-2=6, 10-2=8 => 3, 6, 7, 8. Diff = 5.
        assert_eq!(min_height_diff(&mut arr, k), 5);
    }
    
    #[test]
    fn test_min_height_diff_2() {
        let mut arr = vec![3, 9, 12, 16, 20];
        let k = 3;
        assert_eq!(min_height_diff(&mut arr, k), 11);
    }
}
