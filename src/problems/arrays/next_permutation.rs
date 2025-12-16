/// Next Permutation
/// 
/// Modifies the array to the next lexicographical permutation.
/// Time Complexity: O(N)
/// Space Complexity: O(1)
pub fn next_permutation(nums: &mut [i32]) {
    let n = nums.len();
    if n <= 1 {
        return;
    }

    let mut i = n - 2;
    // 1. Find the first decreasing element from the end
    // Use a while loop with careful index handling because usize can't be negative
    loop {
        if nums[i] < nums[i + 1] {
            break;
        }
        if i == 0 {
            // Entire array is descending, reverse it to get lowest permutation
            nums.reverse();
            return;
        }
        i -= 1;
    }

    // 2. Find the element just larger than nums[i] from the end
    let mut j = n - 1;
    while nums[j] <= nums[i] {
        j -= 1;
    }

    // 3. Swap them
    nums.swap(i, j);

    // 4. Reverse the suffix
    nums[i + 1..].reverse();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_permutation_basic() {
        let mut nums = vec![1, 2, 3];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 3, 2]);
    }
    
    #[test]
    fn test_next_permutation_last() {
        let mut nums = vec![3, 2, 1];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 2, 3]);
    }
    
    #[test]
    fn test_next_permutation_middle() {
        let mut nums = vec![1, 1, 5];
        next_permutation(&mut nums);
        assert_eq!(nums, vec![1, 5, 1]);
    }
}
