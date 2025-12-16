/// Rearrange Array Alternating Positive and Negative
/// 
/// Preserves order. O(1) extra space means O(N^2) time with rotation.
pub fn rearrange_alternate(arr: &mut [i32]) {
    let n = arr.len();
    let mut i = 0;
    
    while i < n {
        if i % 2 == 0 {
            // Expected positive (or >= 0)
            if arr[i] >= 0 {
                i += 1;
                continue;
            }
            
            // Found negative, search for next positive
            if let Some(pos_idx) = find_next(arr, i + 1, true) {
                right_rotate(arr, i, pos_idx);
            } else {
                break; // No more positives
            }
        } else {
            // Expected negative
            if arr[i] < 0 {
                i += 1;
                continue;
            }
            
            // Found positive, search for next negative
            if let Some(neg_idx) = find_next(arr, i + 1, false) {
                right_rotate(arr, i, neg_idx);
            } else {
                break; // No more negatives
            }
        }
        i += 1;
    }
}

fn find_next(arr: &[i32], start: usize, find_positive: bool) -> Option<usize> {
    for k in start..arr.len() {
        if (find_positive && arr[k] >= 0) || (!find_positive && arr[k] < 0) {
            return Some(k);
        }
    }
    None
}

fn right_rotate(arr: &mut [i32], from: usize, to: usize) {
    let temp = arr[to];
    for k in (from + 1..=to).rev() {
        arr[k] = arr[k - 1];
    }
    arr[from] = temp;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rearrange() {
        let mut arr = vec![-5, -2, 5, 2, 4, 7, 1, 8, 0, -8];
        rearrange_alternate(&mut arr);
        // Expected pattern: Pos, Neg, Pos, Neg...
        // Original: -5, -2, 5, 2, 4, 7, 1, 8, 0, -8
        // P: 5, 2, 4, 7, 1, 8, 0
        // N: -5, -2, -8
        // Result: 5, -5, 2, -2, 4, -8, 7, 1, 8, 0
        assert_eq!(arr, vec![5, -5, 2, -2, 4, -8, 7, 1, 8, 0]);
    }
    
    #[test]
    fn test_rearrange_simple() {
        let mut arr = vec![1, 2, 3, -4, -1, 4];
        rearrange_alternate(&mut arr);
        // 1, -4, 2, -1, 3, 4
        assert_eq!(arr, vec![1, -4, 2, -1, 3, 4]);
    }
}
