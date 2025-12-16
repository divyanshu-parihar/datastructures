/// Find Duplicate in Array of N+1 Integers
/// 
/// Uses Floyd's Tortoise and Hare (Cycle Detection) algorithm.
/// Time Complexity: O(n)
/// Space Complexity: O(1)
/// Assumes values are in range [1, n] and there is exactly one duplicate (which may repeat).
pub fn find_duplicate(nums: &[i32]) -> i32 {
    let mut tortoise = nums[0];
    let mut hare = nums[0];

    // Phase 1: Finding the intersection point
    loop {
        tortoise = nums[tortoise as usize];
        hare = nums[nums[hare as usize] as usize];
        if tortoise == hare {
            break;
        }
    }

    // Phase 2: Finding the entrance to the cycle
    let mut ptr1 = nums[0];
    let mut ptr2 = tortoise;
    while ptr1 != ptr2 {
        ptr1 = nums[ptr1 as usize];
        ptr2 = nums[ptr2 as usize];
    }

    ptr1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicate_basic() {
        let nums = vec![1, 3, 4, 2, 2];
        assert_eq!(find_duplicate(&nums), 2);
    }

    #[test]
    fn test_duplicate_multiple() {
        let nums = vec![3, 1, 3, 4, 2];
        assert_eq!(find_duplicate(&nums), 3);
    }
}
