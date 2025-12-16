/// Chocolate Distribution Problem
/// 
/// Find min difference between max and min chocolates in a packet distribution for m students.
/// Time Complexity: O(N log N)
/// Space Complexity: O(1)
pub fn chocolate_distribution(arr: &mut [i32], m: usize) -> i32 {
    let n = arr.len();
    if n == 0 || m == 0 {
        return 0;
    }
    if m > n {
        return -1; // Not possible
    }
    
    arr.sort();
    
    let mut min_diff = i32::MAX;
    
    for i in 0..=(n - m) {
        let diff = arr[i + m - 1] - arr[i];
        if diff < min_diff {
            min_diff = diff;
        }
    }
    
    min_diff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chocolate_distribution() {
        let mut arr = vec![3, 4, 1, 9, 56, 7, 9, 12];
        let m = 5;
        // Sorted: 1, 3, 4, 7, 9, 9, 12, 56
        // Windows of 5:
        // [1, 3, 4, 7, 9] -> diff 8
        // [3, 4, 7, 9, 9] -> diff 6
        // [4, 7, 9, 9, 12] -> diff 8
        // [7, 9, 9, 12, 56] -> diff 49
        assert_eq!(chocolate_distribution(&mut arr, m), 6);
    }
}
