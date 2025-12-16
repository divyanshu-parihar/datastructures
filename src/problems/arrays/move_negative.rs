/// Move Negative Elements to One Side
/// 
/// Moves all negative numbers to the left side and positive to the right.
/// Order is not preserved.
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn move_negative(arr: &mut [i32]) {
    let mut j = 0;
    for i in 0..arr.len() {
        if arr[i] < 0 {
            if i != j {
                arr.swap(i, j);
            }
            j += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_negative_basic() {
        let mut arr = vec![-12, 11, -13, -5, 6, -7, 5, -3, -6];
        move_negative(&mut arr);
        // Verify all negatives are before positives
        let split_idx = arr.iter().position(|&x| x >= 0).unwrap_or(arr.len());
        for i in 0..split_idx {
            assert!(arr[i] < 0);
        }
        for i in split_idx..arr.len() {
            assert!(arr[i] >= 0);
        }
    }

    #[test]
    fn test_move_negative_all_pos() {
        let mut arr = vec![1, 2, 3];
        move_negative(&mut arr);
        assert_eq!(arr, vec![1, 2, 3]);
    }

    #[test]
    fn test_move_negative_all_neg() {
        let mut arr = vec![-1, -2, -3];
        move_negative(&mut arr);
        // Order might change, but all should still be there
        assert_eq!(arr.len(), 3);
        assert!(arr.iter().all(|&x| x < 0));
    }
}
