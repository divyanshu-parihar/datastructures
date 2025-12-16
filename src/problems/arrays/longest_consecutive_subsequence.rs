use std::collections::HashSet;

/// Longest Consecutive Subsequence
/// 
/// Returns the length of the longest consecutive elements sequence.
/// Time Complexity: O(N)
/// Space Complexity: O(N)
pub fn longest_consecutive(nums: &[i32]) -> i32 {
    let set: HashSet<&i32> = nums.iter().collect();
    let mut longest_streak = 0;
    
    for &num in &set {
        // Only start counting if num is the beginning of a sequence
        if !set.contains(&(num - 1)) {
            let mut current_num = *num;
            let mut current_streak = 1;
            
            while set.contains(&(current_num + 1)) {
                current_num += 1;
                current_streak += 1;
            }
            
            longest_streak = std::cmp::max(longest_streak, current_streak);
        }
    }
    
    longest_streak
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_consecutive() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        // 1, 2, 3, 4 => length 4
        assert_eq!(longest_consecutive(&nums), 4);
    }
    
    #[test]
    fn test_longest_consecutive_empty() {
        let nums: Vec<i32> = vec![];
        assert_eq!(longest_consecutive(&nums), 0);
    }
}
