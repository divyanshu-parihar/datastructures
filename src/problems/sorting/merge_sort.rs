/// Merge Sort Implementation
/// 
/// Sorts a mutable slice in ascending order using the Merge Sort algorithm.
/// Time Complexity: O(n log n)
/// Space Complexity: O(n)
pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }
    
    let mid = n / 2;
    let (left, right) = arr.split_at_mut(mid);

    merge_sort(left);
    merge_sort(right);

    let mut new_state: Vec<T> = Vec::with_capacity(n);
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            new_state.push(left[i].clone());
            i += 1;
        } else {
            new_state.push(right[j].clone());
            j += 1;
        }
    }

    new_state.extend_from_slice(&left[i..]);
    new_state.extend_from_slice(&right[j..]);

    arr.clone_from_slice(&new_state);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort_basic() {
        let mut arr = vec![64, 25, 12, 22, 11];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 64]);
    }

    #[test]
    fn test_merge_sort_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort_duplicates() {
        let mut arr = vec![3, 1, 2, 3, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 3]);
    }

    #[test]
    fn test_merge_sort_empty() {
        let mut arr: Vec<i32> = vec![];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }
}