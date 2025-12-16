use std::fmt::Debug;

/// Reverse the array
///
/// Reverses the elements of a mutable slice in place.
/// Time Complexity: O(n)
/// Space Complexity: O(1)
pub fn reverse_array<T>(arr: &mut [T])
where
    T: Debug,
{
    let length = arr.len();
    if length <= 1 {
        return;
    }
    let mut first = 0;
    let mut second = length - 1;

    while (first <= second) {
        // swap the elements
        arr.swap(first, second);
        first += 1;
        second -= 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_basic() {
        let mut arr = vec![1, 2, 3, 4, 5];
        reverse_array(&mut arr);
        assert_eq!(arr, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_reverse_empty() {
        let mut arr: Vec<i32> = vec![];
        reverse_array(&mut arr);
        assert_eq!(arr, vec![]);
    }

    #[test]
    fn test_reverse_single() {
        let mut arr = vec![1];
        reverse_array(&mut arr);
        assert_eq!(arr, vec![1]);
    }
}
