/// Find Triplet that sums to a given value
/// 
/// Returns true if such triplet exists.
/// Time Complexity: O(N^2)
/// Space Complexity: O(1)
pub fn triplet_sum(arr: &mut [i32], target: i32) -> bool {
    let n = arr.len();
    if n < 3 {
        return false;
    }
    
    arr.sort();
    
    for i in 0..n-2 {
        let mut left = i + 1;
        let mut right = n - 1;
        
        while left < right {
            let sum = arr[i] + arr[left] + arr[right];
            match sum.cmp(&target) {
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Less => left += 1,
                std::cmp::Ordering::Greater => right -= 1,
            }
        }
    }
    
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triplet_sum() {
        let mut arr = vec![1, 4, 45, 6, 10, 8];
        let target = 22;
        // 4 + 8 + 10 = 22 or 6 + ? 
        // Sorted: 1, 4, 6, 8, 10, 45
        // 4+8+10=22. Yes.
        assert!(triplet_sum(&mut arr, target));
    }
    
    #[test]
    fn test_triplet_sum_false() {
        let mut arr = vec![1, 2, 3];
        let target = 10;
        assert!(!triplet_sum(&mut arr, target));
    }
}
