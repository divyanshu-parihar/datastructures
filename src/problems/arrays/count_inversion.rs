/// Count Inversions
/// 
/// Returns the number of inversions.
/// Time Complexity: O(n log n)
/// Space Complexity: O(n)
pub fn count_inversions(arr: &mut [i32]) -> usize {
    merge_sort_count(arr)
}

fn merge_sort_count(arr: &mut [i32]) -> usize {
    let n = arr.len();
    if n <= 1 {
        return 0;
    }

    let mid = n / 2;
    let (left, right) = arr.split_at_mut(mid);
    
    let mut count = merge_sort_count(left) + merge_sort_count(right);
    
    let mut temp = Vec::with_capacity(n);
    let mut i = 0;
    let mut j = 0;
    
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            temp.push(left[i]);
            i += 1;
        } else {
            // If left[i] > right[j], then left[i] and all subsequent elements in left
            // are greater than right[j], forming inversions.
            temp.push(right[j]);
            count += left.len() - i;
            j += 1;
        }
    }
    
    temp.extend_from_slice(&left[i..]);
    temp.extend_from_slice(&right[j..]);
    
    arr.copy_from_slice(&temp);
    
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inversions_basic() {
        let mut arr = vec![2, 4, 1, 3, 5];
        // Inversions: (2,1), (4,1), (4,3) => 3
        assert_eq!(count_inversions(&mut arr), 3);
    }
    
    #[test]
    fn test_inversions_reverse() {
        let mut arr = vec![5, 4, 3, 2, 1];
        // 4 + 3 + 2 + 1 = 10
        assert_eq!(count_inversions(&mut arr), 10);
    }
    
    #[test]
    fn test_inversions_sorted() {
        let mut arr = vec![1, 2, 3];
        assert_eq!(count_inversions(&mut arr), 0);
    }
}
