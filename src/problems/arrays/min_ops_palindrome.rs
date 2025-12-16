/// Minimum Operations to Make Array Palindrome
/// 
/// Returns number of merge operations.
/// Time Complexity: O(N)
/// Space Complexity: O(1)
pub fn min_ops_palindrome(arr: &[i32]) -> i32 {
    let mut ans = 0;
    let mut i = 0;
    let mut j = arr.len() - 1;
    
    // Since we need to modify values (merge), we ideally need a mutable array or simulate it.
    // Simulating with variables is easier.
    // But we need to track accumulated sums.
    // Wait, the input is immutable slice. We should probably accept a Vec or make a copy to be correct, 
    // OR just track the *current* values at i and j (which might be sums of merged elements).
    // Let's use internal variables to track current values at ends.
    
    // However, tracking just indices isn't enough if we modify.
    // The standard algorithm modifies the array.
    // I'll clone it to a Vec.
    let mut arr = arr.to_vec();
    
    while i <= j {
        if arr[i] == arr[j] {
            i += 1;
            if j == 0 { break; } // prevent underflow
            j -= 1;
        } else if arr[i] < arr[j] {
            i += 1;
            arr[i] += arr[i-1];
            ans += 1;
        } else {
            j -= 1;
            arr[j] += arr[j+1];
            ans += 1;
        }
    }
    
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_ops_palindrome() {
        let arr = vec![1, 4, 5, 1];
        // Merge 4, 5 -> 9. [1, 9, 1]. Ops: 1.
        assert_eq!(min_ops_palindrome(&arr), 1);
    }
    
    #[test]
    fn test_min_ops_palindrome_2() {
        let arr = vec![11, 14, 15, 99];
        // 11 != 99. 11 < 99. Merge 11, 14 -> 25. Ans 1.
        // 25 != 99. 25 < 99. Merge 25, 15 -> 40. Ans 2.
        // 40 != 99. 40 < 99. Merge 40, 99... wait.
        // Eventually everything merges to one number.
        // Logic:
        // [11, 14, 15, 99]
        // i=0, j=3. 11 < 99. i++, arr[1] += 11 -> [11, 25, 15, 99]. ans=1.
        // i=1, j=3. 25 < 99. i++, arr[2] += 25 -> [11, 25, 40, 99]. ans=2.
        // i=2, j=3. 40 < 99. i++, arr[3] += 40 -> [11, 25, 40, 139]. ans=3.
        // i=3, j=3. Equal.
        // Total 3.
        assert_eq!(min_ops_palindrome(&arr), 3);
    }
}
