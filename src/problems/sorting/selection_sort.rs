/// Selection Sort Implementation
///
/// Sorts a mutable slice in ascending order.
/// Time Complexity: O(n^2)
/// Space Complexity: O(1)
pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    // implementation for the selection sort
    for i in 0..arr.len() {
        let mut index = i;
        for j in i..arr.len() {
            match arr[index].cmp(&arr[j]) {
                std::cmp::Ordering::Greater => {
                    index = j;
                }
                _ => {}
            }
        }
        // swap values with the index and min value
        arr.swap(index, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort_basic() {
        let mut arr = vec![64, 25, 12, 22, 11];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![11, 12, 22, 25, 64]);
    }

    #[test]
    fn test_selection_sort_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_selection_sort_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_selection_sort_duplicates() {
        let mut arr = vec![3, 1, 2, 3, 1];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 3]);
    }

    #[test]
    fn test_selection_sort_empty() {
        let mut arr: Vec<i32> = vec![];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![]);
    }
}
