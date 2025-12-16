/// Find Minimum and Maximum
///
/// Returns a tuple (min, max) from the slice.
/// Returns None if the slice is empty.
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn find_min_max<T: Ord + Copy>(arr: &[T]) -> Option<(T, T)> {
    if arr.len() == 0 {
        return None;
    }
    let mut min = arr[0];
    let mut max = arr[0];
    for el in arr {
        min = min.min(el.clone());
        max = max.max(el.clone())
    }
    Some((min, max))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_max_basic() {
        let arr = vec![1, 5, 3, 9, 2];
        assert_eq!(find_min_max(&arr), Some((1, 9)));
    }

    #[test]
    fn test_min_max_empty() {
        let arr: Vec<i32> = vec![];
        assert_eq!(find_min_max(&arr), None);
    }

    #[test]
    fn test_min_max_single() {
        let arr = vec![42];
        assert_eq!(find_min_max(&arr), Some((42, 42)));
    }
}
